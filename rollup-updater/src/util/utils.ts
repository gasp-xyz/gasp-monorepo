import * as util from "node:util";
import {ApiPromise} from "@polkadot/api";
import {decodeAbiParameters, publicActions, type PublicClient, type WalletClient} from "viem";
import {LIMIT, MANGATA_CONTRACT_ADDRESS, ROLLDOWN_ABI, VERBOSE} from "../common/constants.js";
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


async function getPendingUpdate(api: ApiPromise, blockHash: Uint8Array) {
    return await api.rpc.rolldown.pending_l2_requests('Ethereum', blockHash);
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
    print(`HASH: ${blockHash} `);
    const pendingUpdates = await getPendingUpdate(api, blockHash);

    if (pendingUpdates.isEmpty){
        print("PendingUpdates is empty");
        return null;
    }

    const l2Update = getDecodedData("update_l1_from_l2", pendingUpdates.toHex())

    if (VERBOSE) {
        print(`l2Update:  ${JSON.stringify(l2Update, null, 2)}`);
    }

    const lastSubmittedId = (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "lastProcessedUpdate_origin_l2",
    })) as number;

    print(`L1::lastSubmittedId: ${lastSubmittedId}`);

    const update = filterUpdates(l2Update, lastSubmittedId);

    if (VERBOSE) {
        console.log(`filtered l2Update:  ${JSON.stringify(update, null, 2)}`);
    }

    if (getCountRequest(l2Update) === 0) {
        return null
    } else {
        const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(publicClient)

        const {request} = await publicClient.simulateContract({
            account: ethAccount,
            chain: getChain(),
            abi: ROLLDOWN_ABI,
            address: MANGATA_CONTRACT_ADDRESS,
            functionName: "update_l1_from_l2",
            args: update,
            maxFeePerGas: maxFeeInWei,
            maxPriorityFeePerGas: maxPriorityFeePerGasInWei
        })

        print(`Write Contract Request: ${JSON.stringify(request.args)}`)

        return await walletClient.writeContract(request)
    }
}

function print(data: any) {
    console.log(util.inspect(data, { depth: null }));
}

export {
    print,
    sendUpdateToL1
}
