
import {
  ABI,
  MANGATA_CONTRACT_ADDRESS,
} from "../common/constants.js";

import { PrivateKeyAccount, createPublicClient, createWalletClient } from "viem";
import { type PublicClient } from "viem";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import {
  http,
  webSocket,
} from "viem";
import { Withdrawal } from "../common/withdrawal.js";
import { privateKeyToAccount } from "viem/accounts";
import { anvil } from "viem/chains";
import { isEqual } from "../utils/index.js";
import { estimateMaxPriorityFeePerGas } from "viem/actions";

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


function toViemFormat(withdrawal: Withdrawal): unknown[]  {
  return [
    [1, withdrawal.requestId], 
    u8aToHex(withdrawal.withdrawalRecipient, 160), 
    u8aToHex(withdrawal.tokenAddress, 160), 
    withdrawal.amount, 
    withdrawal.ferryTip];
}

function minBigInt(lhs: bigint, rhs: bigint): bigint {
  return [lhs, rhs].reduce((min, current) => current < min ? current : min);
}

interface L1Interface {
  isRolldownDeployed(): Promise<boolean>;
  getLatestRequestId(): Promise<bigint | null>;
  getBalance(account: Uint8Array, tokenAddress: Uint8Array): Promise<bigint | null>;
  getNativeTokenAddress(): Promise<Uint8Array>;

  isClosed(hash: Uint8Array): Promise<boolean>;
  isFerried(hash: Uint8Array): Promise<boolean>;
  getFerry(hash: Uint8Array): Promise<Uint8Array | null>;

  ferry(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<boolean>;
  close(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<boolean>;
}

class L1Api implements L1Interface {
  client!: PublicClient;
  transport: any;

  constructor(uri: string) {
    if (uri.startsWith("ws")) {
      this.transport = webSocket(uri, { retryCount: 5 });
      this.client = createPublicClient({
        transport: this.transport
      });
    } else if (uri.startsWith("http")) {
      this.transport = http(uri, { retryCount: 5 });
      this.client = createPublicClient({
        transport: this.transport,
      });
    } else {
      throw new Error("Invalid uri");
    }
  }

  async getFerry(hash: Uint8Array): Promise<Uint8Array | null> {
    const closedStatus = u8aToHex( await this.getClosedStatus());
    const zeros = "0x0000000000000000000000000000000000000000";

    const status = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "processedL2Requests",
      args: [u8aToHex(hash)],
      blockTag: "latest"
    }) as string;

    return (status !== null && status !== closedStatus && status !== zeros) ? hexToU8a(status) : null;
  }

  async getNativeTokenAddress(): Promise<Uint8Array> {
    let val = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "NATIVE_TOKEN_ADDRESS",
      blockTag: "latest"
    });
    return hexToU8a(val as any);
  }

  async getBalance(tokenAddress: Uint8Array, account: Uint8Array): Promise<bigint | null> {
    const nativeTokenAddress = await this.getNativeTokenAddress();
    if (isEqual(nativeTokenAddress , tokenAddress)) {
      return this.client.getBalance({address: u8aToHex(account)})
    } else {
    try {
      return BigInt(await this.client.readContract({
        address: u8aToHex(tokenAddress),
        abi: [{
          "constant": true,
          "inputs": [{ "name": "owner", "type": "address" }],
          "name": "balanceOf",
          "outputs": [{ "name": "balance", "type": "uint256" }],
          "type": "function",
        }],
        functionName: "balanceOf",
        args: [u8aToHex(account)],
        blockTag: "latest"
      }) as any);
    } catch (e) {
      return null;
    }
    }
  }

  async getClosedStatus(): Promise<Uint8Array> {
    let closedStatus = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "CLOSED",
      blockTag: "latest"
    });
    return hexToU8a(closedStatus as string);

  }

  async isClosed(hash: Uint8Array): Promise<boolean> {
    const closedStatus = u8aToHex( await this.getClosedStatus());
    let status = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "processedL2Requests",
      args: [u8aToHex(hash)],
      blockTag: "latest"
    });
    return status === closedStatus;
  }

  async isFerried(hash: Uint8Array): Promise<boolean> {
    const closedStatus = u8aToHex( await this.getClosedStatus());
    const zeros = "0x0000000000000000000000000000000000000000";

    let status = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "processedL2Requests",
      args: [u8aToHex(hash)],
      blockTag: "latest"
    });

    return status !== closedStatus && status !== zeros;
  }

  async isRolldownDeployed(): Promise<boolean> {
    const code = await this.client.getCode({
      address: MANGATA_CONTRACT_ADDRESS,
      blockTag: "latest",
    });
    return code !== undefined && code !== "0x";
  }

  async getLatestRequestId(): Promise<bigint | null> {
    const value = BigInt(await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "getMerkleRootsLength",
      blockTag: "latest"
    }) as any);

    if (value === 0n) {
      return null;
    }else{

      const lastHash = await this.client.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ABI,
        functionName: "roots",
        args: [value-1n],
        blockTag: "latest"
      });


      const range = await this.client.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ABI,
        functionName: "merkleRootRange",
        args: [lastHash],
        blockTag: "latest"
      });
      return (range as any)[1];
    }
  }

  async ferry(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<boolean>{
    const acc: PrivateKeyAccount = privateKeyToAccount(u8aToHex(privateKey) as `0x{string}`);
  const wc = createWalletClient({
      account: acc,
      chain: anvil,
      transport: this.transport,
    });

    // TODO: submit as a batch
    const approveRequest = await this.client.simulateContract({
      account: acc,
      address: u8aToHex(withdrawal.tokenAddress),
      abi: [{
        "constant": true,
        "inputs": [{ "name": "spender", "type": "address" }, { "name": "amount", "type": "uint256" }],
        "name": "approve",
        "outputs": [{ "name": "status", "type": "bool" }],
        "type": "function",
      }],
      functionName: "approve",
      args: [MANGATA_CONTRACT_ADDRESS, withdrawal.amount]
    });
    const approvetxHash = await wc.writeContract(approveRequest.request);
    await this.client.waitForTransactionReceipt({ hash: approvetxHash });


    const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(this.client);
    const ferryRequest = await this.client.simulateContract({
      account: acc,
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "ferry_withdrawal",
      args: [toViemFormat(withdrawal)],
      maxFeePerGas: maxFeeInWei,
      maxPriorityFeePerGas: maxPriorityFeePerGasInWei
    });

    const ferrytxHash = await wc.writeContract(ferryRequest.request);
    const status = await this.client.waitForTransactionReceipt({ hash: ferrytxHash });
    return status.status === "success";
  }


  async close(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<boolean>{
    const acc: PrivateKeyAccount = privateKeyToAccount(u8aToHex(privateKey) as `0x{string}`);
    const wc = createWalletClient({
      account: acc,
      chain: anvil,
      transport: this.transport,
    });


    const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(this.client);
    const ferryRequest = await this.client.simulateContract({
      account: acc,
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "close_withdrawal",
      args: [toViemFormat(withdrawal), u8aToHex(withdrawal.hash, 256), [u8aToHex(withdrawal.hash, 256)]],
      maxFeePerGas: maxFeeInWei,
      maxPriorityFeePerGas: maxPriorityFeePerGasInWei
    });

    const ferrytxHash = await wc.writeContract(ferryRequest.request);
    const status = await this.client.waitForTransactionReceipt({ hash: ferrytxHash });
    return status.status === "success";
  }
}


export { L1Interface, L1Api, toViemFormat };
