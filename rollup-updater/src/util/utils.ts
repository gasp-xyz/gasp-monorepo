import * as util from "node:util";
import {ApiPromise} from "@polkadot/api";
import {decodeAbiParameters, publicActions, type PublicClient, type WalletClient, type Hash, keccak256} from "viem";
import {LIMIT, MANGATA_CONTRACT_ADDRESS, ROLLDOWN_METADATA, ROLLDOWN_ABI, L1_CHAIN} from "../common/constants.js";
import {ethAccount, getChain} from "../viem/client.js";
import {Cancel, L2Update, RequestResult, Withdrawal} from "../common/types.js";
import {estimateMaxPriorityFeePerGas} from "viem/actions";


export function getMinRequestId(l2Update: Array<L2Update>) {
    let minId = Math.min(...[
        l2Update[0].withdrawals,
        l2Update[0].cancels,
        l2Update[0].results
    ].flat()
        .map(function(item) {
            return Number(item.requestId.id);
        }))

    return minId === Infinity ? null : minId
}

export async function getLatestRequestIdSubmittedToL1(publicClient: PublicClient) {
    return (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "lastProcessedUpdate_origin_l2",
    })) as bigint;
}

function filterUpdates(l2Update: Array<L2Update>, lastSubmittedId: number) {

    l2Update[0].withdrawals = l2Update[0].withdrawals.filter((elem: Withdrawal) => elem.requestId.id > lastSubmittedId);
    l2Update[0].cancels = l2Update[0].cancels.filter((elem: Cancel) => elem.requestId.id > lastSubmittedId);
    l2Update[0].results = l2Update[0].results.filter((elem: RequestResult) =>  elem.requestId.id > lastSubmittedId);

    const minId = getMinRequestId(l2Update);
    if (minId == null) {
        return l2Update;
    }

    const maxAmountOfUpdates = LIMIT
    if (maxAmountOfUpdates > 0) {
        l2Update[0].withdrawals = l2Update[0].withdrawals.filter((elem: Withdrawal) => elem.requestId.id < BigInt(minId + maxAmountOfUpdates));
        l2Update[0].cancels = l2Update[0].cancels.filter((elem: Cancel) => elem.requestId.id < BigInt(minId + maxAmountOfUpdates));
        l2Update[0].results = l2Update[0].results.filter((elem: RequestResult) => elem.requestId.id < BigInt(minId + maxAmountOfUpdates));
        return l2Update;
    } else {
        return l2Update;
    }

}

async function getLastBatchId(api: ApiPromise, blockHash: Uint8Array) {
    const chain = api.createType('Chain', L1_CHAIN);
    let apiAt = await api.at(blockHash);
    let last_batch = await apiAt.query.rolldown.l2RequestsBatchLast();
    let specificL1LastBatch = last_batch.toHuman()[L1_CHAIN];
    if (specificL1LastBatch == undefined ){
      return null
    }else{
        return (specificL1LastBatch as any)[1];
    }
}

async function findBatchWithNewUpdates(api: ApiPromise, publicClient: PublicClient, blockHash: Uint8Array) {

    let batchId = await getLastBatchId(api, blockHash)
    if (batchId == null){
      return null;
    }

    const lastSubmittedId = await getLatestRequestIdSubmittedToL1(publicClient);
    const nextRequestId = lastSubmittedId + 1n;

    while (batchId > 0) {
      let batch = await api.query.rolldown.l2RequestsBatch([L1_CHAIN, batchId]);
      let rangeStart = BigInt((batch.toHuman() as any)[1][0]);
      let rangeStop = BigInt((batch.toHuman() as any)[1][1]);
      if (rangeStart <= nextRequestId && rangeStop >= nextRequestId) {
        return [rangeStart, rangeStop];
      }
      batchId -= 1;
    }

    console.log(`couldnt find any batch with requestId: ${nextRequestId}`);
    return null;
}



async function getPendingUpdate(api: ApiPromise, blockHash: Uint8Array) {
    const chain = api.createType('Chain', L1_CHAIN);
    return await api.rpc.rolldown.pending_l2_requests(chain, blockHash);
}

function getDecodedData(methodName: string, pendingUpdates: `0x${string}`): Array<L2Update> {
    return decodeAbiParameters(
        ROLLDOWN_ABI.find((e: any) => e.name === methodName)!.inputs,
        pendingUpdates,
    ) as unknown as Array<L2Update>;
}

function getCountRequest(l2Update: Array<L2Update>) {
    return l2Update[0].withdrawals.length +
        l2Update[0].cancels.length +
        l2Update[0].results.length;
}

async function estimateGasInWei(publicClient: PublicClient) {
    // https://www.blocknative.com/blog/eip-1559-fees
    // We do not want VIEM estimate we would like to make our own estimate
    // based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee

    // Max Fee = maxFeePerGas (viem)
    // Max Priority Fee = maxPriorityFeePerGas (viem)

    const baseFeeInWei = await publicClient.getGasPrice()

    const maxPriorityFeePerGasInWei =  await estimateMaxPriorityFeePerGas(publicClient)

    const maxFeeInWei = BigInt(2) * BigInt(baseFeeInWei) + BigInt(maxPriorityFeePerGasInWei)

    return {maxFeeInWei, maxPriorityFeePerGasInWei}
}

async function sendUpdateToL1(
    api: ApiPromise,
    walletClient: WalletClient,
    publicClient: PublicClient,
    blockHash: Uint8Array,
) {
    let requestsRange = await findBatchWithNewUpdates(api, publicClient, blockHash);

    if (requestsRange == null){
      return null;
    }
    const rangeStart = requestsRange[0];
    const rangeEnd = requestsRange[1];

    let root = await api.rpc.rolldown.get_merkle_root(L1_CHAIN, [rangeStart, rangeEnd]);
    if (root.toString() == "0x0000000000000000000000000000000000000000000000000000000000000000") {
      return null
    }

    const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(publicClient)
    const {request} = await publicClient.simulateContract({
        account: ethAccount,
        chain: getChain(),
        abi: ROLLDOWN_ABI,
        address: MANGATA_CONTRACT_ADDRESS,
        functionName: "update_l1_from_l2",
        args: [root.toHex(), [rangeStart, rangeEnd]],
        maxFeePerGas: maxFeeInWei,
        maxPriorityFeePerGas: maxPriorityFeePerGasInWei
    })
    const txHash = await walletClient.writeContract(request);
    const result = await publicClient.waitForTransactionReceipt({ hash: txHash });
    console.log(`#${result.blockNumber} ${result.transactionHash} : ${result.status}`);
    return requestsRange
}

async function findMerkleRange(
    publicClient: PublicClient,
    requestId: bigint
) {
    return (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "find_l2_batch",
        args: [requestId],
    }));
}

async function isL2RequestAlreadyExecuted(
    publicClient: PublicClient,
    requestId: bigint
) {
    return (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "processedL2Requests",
        args: [requestId],
    }));
}


export async function closeWithdrawals(
    api: ApiPromise,
    walletClient: WalletClient,
    publicClient: PublicClient,
    lastExecutedRequestId: bigint
)  {
    const lastSubmittedId = (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "lastProcessedUpdate_origin_l2",
    })) as bigint;

    let indexes = [];
    for( let i = lastExecutedRequestId + 1n; i <= lastSubmittedId; i++){
      indexes.push(i);
    }

    if (indexes.length == 0){
      return lastExecutedRequestId;
    }

    for (let withdrawalRequestId of indexes){
      if (await isL2RequestAlreadyExecuted(publicClient, withdrawalRequestId)){
        console.log(`withdrawal ${withdrawalRequestId} already executed - ignoring...`);
        continue;
      };
      let range = await findMerkleRange(publicClient, withdrawalRequestId)
      const rangeStart = (range as any).start;
      const rangeEnd = (range as any).end;
      let encodedWithdrawal = await api.rpc.rolldown.get_abi_encoded_l2_request(L1_CHAIN, withdrawalRequestId);
      const chain = api.createType('Chain', L1_CHAIN);
      console.log(`chain: ${chain} range: [${rangeStart}, ${rangeEnd}] withdrawalRequestId: ${withdrawalRequestId} encodedWithdrawal: ${encodedWithdrawal}`);
      let root = await api.rpc.rolldown.get_merkle_root(chain, [rangeStart, rangeEnd]);
      let proof = await api.rpc.rolldown.get_merkle_proof(chain, [rangeStart, rangeEnd], withdrawalRequestId);
      const withdrawal = decodeAbiParameters((ROLLDOWN_METADATA as any).output.abi.find((e: any) => e.name === "close_withdrawal")!.inputs[0].components, encodedWithdrawal.toHex())
      const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(publicClient)
      const {request} = await publicClient.simulateContract({
          account: ethAccount,
          chain: getChain(),
          abi: ROLLDOWN_ABI,
          address: MANGATA_CONTRACT_ADDRESS,
          functionName: "close_withdrawal",
          // TODO: it should be possible to pass abi encoded withdrawal fetched from gasp directly (without deserialization)
          args: [withdrawal, root.toHuman(), proof.toHuman()],
          maxFeePerGas: maxFeeInWei,
          maxPriorityFeePerGas: maxPriorityFeePerGasInWei
      })
      const txHash = await walletClient.writeContract(request);
      const result = await publicClient.waitForTransactionReceipt({ hash: txHash });
      console.log(`closing withdrawal ${withdrawalRequestId}: tx:${result.transactionHash} - ${result.status}`);
      lastExecutedRequestId = withdrawalRequestId;
    }
    return lastExecutedRequestId;
}

function print(data: any) {
    console.log(util.inspect(data, { depth: null }));
}

export {
    print,
    sendUpdateToL1
}
