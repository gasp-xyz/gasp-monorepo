
import {
  Deposit
} from "../common/deposit.js";

import {
  ABI,
  MANGATA_CONTRACT_ADDRESS,
} from "../common/constants.js";

import { createPublicClient } from "viem";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import {
  http,
  webSocket,
} from "viem";
import { Withdrawal } from "../common/withdrawal.js";

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

  getFerriedAndReadyToClose(rangeStart: bigint, rangeEnd: bigint): Promise<Withdrawal[]>;
}

class L1Api implements L1Interface {
  client!: PublicClient;
  delay: bigint;

  constructor(uri: string, delay: bigint) {
    this.delay = delay;
    if (uri.startsWith("ws")) {
      this.client = createPublicClient({
        transport: webSocket(uri, { retryCount: 5 }),
      });
    } else if (uri.startsWith("http")) {
      this.client = createPublicClient({
        transport: http(uri, { retryCount: 5 }),
      });
    } else {
      throw new Error("Invalid uri");
    }
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
    if (u8aToHex(nativeTokenAddress) === u8aToHex(tokenAddress)) {
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
    const blockNumber = await this.client.getBlockNumber();
    if (blockNumber < this.delay) {
      return false;
    }
    const blockToReadAt = blockNumber - this.delay;

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
      console.info(range);
      return null;
    }
  }

  async getFerriedAndReadyToClose(rangeStart: bigint, rangeEnd: bigint): Promise<Withdrawal[]> {
    const latest = await this.getLatestRequestId();
    if (latest == null) {
      return [];
    }

    const end = minBigInt(rangeEnd, latest);
    const value = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "counter",
      blockTag: "latest"
    });

    return [];
  }

}

export { L1Interface, L1Api };
