import util from "node:util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { webSocket, http, PrivateKeyAccount, createWalletClient } from "viem";
import type { ApiDecoration } from "@polkadot/api/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import { privateKeyToAccount } from "viem/accounts";
import { anvil } from "viem/chains";
import type { StorageKey } from "@polkadot/types";
import type { Option, u128, u32 } from "@polkadot/types-codec";
import { type PublicClientConfig, createPublicClient } from "viem";
import type {
  FrameSystemEventRecord,
  PalletRolldownMessagesChain,
  PalletRolldownMessagesL1Update,
  SpRuntimeAccountAccountId20,
  PalletRolldownMessagesDeposit,
  MangataTypesAssetsL1Asset,
  OrmlTokensAccountData,
} from "@polkadot/types/lookup";
import type { ITuple } from "@polkadot/types/types";
import { Mangata, type MangataGenericEvent, signTx } from "gasp-sdk";

import "gasp-types";

import type { H256 } from "@polkadot/types/interfaces/runtime";
import { hexToU8a } from "@polkadot/util";
import type { KeypairType } from "@polkadot/util-crypto/types";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import {
  ABI,
  L1_CHAIN,
  MIN_PROFIT,
  MANGATA_CONTRACT_ADDRESS,
} from "../common/constants.js";

async function asyncFilter(arr: Deposit[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => { return results[index];});
}

interface Deposit {
  readonly requestId: bigint;
  readonly depositRecipient: Uint8Array;
  readonly tokenAddress: Uint8Array;
  readonly amount: bigint;
  readonly timeStamp: bigint;
  readonly ferryTip: bigint;
}

class Ferry  {
  l1: L1Interface;
  l2: L2Interface;
  me: Uint8Array;
  txCost: bigint;
  minProfit: bigint;

  constructor(me: Uint8Array, l1: L1Interface, l2: L2Interface, txCost: bigint, minProfit: bigint) {
    this.me = me;
    this.l1 = l1;
    this.l2 = l2;
    this.txCost = txCost;
    this.minProfit = minProfit;
  }

  async getPendingDeposits() : Promise<Deposit[]> {
    const start = await this.l1.getLatestRequestId();
    const end = await this.l2.getLastProcessedRequestId();
    if (start === null) {
      return Promise.resolve([]);
    }
    const deposits = await this.l1.getDeposits(start, end);

    return await asyncFilter(deposits, async (elem: Deposit) => {
      const hash = await this.l1.getDepostiHash(elem.requestId);
      const isFerried = (await this.l2.isFerried(hash));
      const isExecuted = (await this.l2.isExecuted(elem.requestId));
      return !isFerried && !isExecuted
    })
  }

  async rateDeposits(deposits: Deposit[]) : Promise<Deposit[]> {
    let nativeTokenAddress = await this.l2.getNativeTokenAddress();
    let balances = await this.l2.getBalances(this.me);
    const result = deposits
    .filter((deposit) => { 
      if (deposit.amount < deposit.ferryTip) {
        return false;
      }

      const transferAmount = deposit.amount - deposit.ferryTip;

      if (balances.has(deposit.tokenAddress)) {
        if (deposit.tokenAddress === nativeTokenAddress ){
          return balances.get(deposit.tokenAddress)! >= transferAmount + this.txCost;
        }else {
          return balances.get(deposit.tokenAddress)! >= transferAmount;
        }
      } else {
        return false;
      }
    })
    .sort((a, b) => {
      if (a.ferryTip > b.ferryTip) {
        return -1;
      } else if (a.ferryTip == b.ferryTip) {
        if (a.amount > b.amount ){
          return 1;
        } else if (a.amount > b.amount ){
          return 0;
        }else {
          return -1;
        }
      } else {
        return 1;
      }
    });

    return await asyncFilter(result, async (elem: Deposit) => {
      return (await this.l2.valutateToken(elem.tokenAddress, elem.ferryTip)) >= this.minProfit;
    });
  }
}

async function dummyDeposit(uri: string) {
    const ANVIL_TEST_ACCOUNT = "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";
    const transport = webSocket(uri, { retryCount: 5 });
    const publicClient = createPublicClient({
      transport
    });

    const acc: PrivateKeyAccount = privateKeyToAccount(
      ANVIL_TEST_ACCOUNT,
    );

    const { request } = await publicClient.simulateContract({
      account: acc,
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "deposit_native",
      value: BigInt(123456789),
    });

  const wc = createWalletClient({
    account: acc,
    chain: anvil,
    transport
  });
  return await wc.writeContract(request);
}

interface L1Interface {
  isRolldownDeployed(): Promise<boolean>;
  getLatestRequestId(): Promise<bigint|null>;
  getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]>;
  getDepostiHash(requestId: bigint): Promise<Uint8Array>;
}

interface L2Interface {
  getBalances(address: Uint8Array): Promise<Map<Uint8Array, bigint>>;
  ferryDeposits(deposit: Deposit): Promise<boolean>;
  getLastProcessedRequestId(): Promise<bigint>;
  getLastProcessedRequestId(): Promise<bigint>;
  isExecuted(depositId: bigint): Promise<boolean>;
  isFerried(depositId: Uint8Array): Promise<boolean>;
  getNativeTokenAddress(): Promise<Uint8Array>;
  valutateToken(tokenAddress: Uint8Array, amount: bigint): Promise<bigint>;
}

class L1Api implements L1Interface {
  client!: PublicClient;

  constructor(uri: string) {
    if (uri.startsWith("ws")) {
      this.client = createPublicClient({
        transport: webSocket(uri, { retryCount: 5 })
      });
    } else if (uri.startsWith("http")) {
      this.client = createPublicClient({
        transport: http(uri, { retryCount: 5 })
      });
    } else {
      throw new Error("Invalid uri");
    }
  }


  async getDepostiHash(requestId: bigint): Promise<Uint8Array> {
    const value = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "deposits",
      blockTag: "finalized",
      args: [requestId],
    });
    const hash = encodeAbiParameters(
      ABI.find((e: any) => e!.name === "deposits")!.outputs!,
      [...value as any],
    );
    return hexToU8a(keccak256(hexToU8a(hash)));
  }

  async getLatestRequestId(): Promise<bigint| null> {
    const value = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "counter",
      blockTag: "finalized",
    });
    const reqId = BigInt(value as any) - 1n
    if (reqId < 1n) {
      return null
    }
    return reqId;
  }

  async getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]> {
    const selector = "getPendingRequests"; 
    const contractData = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: selector,
      args: [rangeStart, rangeEnd],
    });
    // console.info(util.inspect((contractData as any).pendingDeposits.length));
    console.info(MANGATA_CONTRACT_ADDRESS);
    console.info(util.inspect((contractData as any)));


    return (contractData as any).pendingDeposits.map((deposit: any) => {
      return {
        requestId: deposit.requestId.id,
        depositRecipient: deposit.depositRecipient,
        tokenAddress: deposit.tokenAddress,
        amount: deposit.amount,
        timeStamp: deposit.timeStamp,
        ferryTip: deposit.ferryTip,
      };
    }
    );
  }

  async isRolldownDeployed(): Promise<boolean> {
    const code = await this.client.getCode({ address: MANGATA_CONTRACT_ADDRESS, blockTag: "finalized" });
    return code !== '0x';
  }
}


class L2Api implements L2Interface {
  api!: ApiPromise;
  keyring!: KeyringPair;

  constructor(api: ApiPromise) {
    this.api = api;
  }

  async valutateToken(tokenAddress: Uint8Array, amount: bigint): Promise<bigint> {
    let asset: MangataTypesAssetsL1Asset = this.api.createType("MangataTypesAssetsL1Asset", {"Ethereum": tokenAddress});
    console.info(asset.toString());
    let tokenId = await this.api.query.assetRegistry.l1AssetToId(asset);
    console.info(tokenId.toString());
    if (tokenId.isNone) {
      return 0n;
    }

    if (tokenId.unwrap().toNumber() == 0) {
      return amount;
    } else {
      return BigInt((await this.api.rpc.xyk.calculate_sell_price(tokenId.unwrap(), 0, amount)).toString());
    }
  }

  async getNativeTokenAddress(): Promise<Uint8Array> {
    return (await this.api.query.assetRegistry.idToL1Asset(0)).unwrap().asEthereum.toU8a();
  }

  async getBalances(address: Uint8Array): Promise<Map<Uint8Array, bigint>> {
    const assetMapping = await this.api.query.assetRegistry.idToL1Asset.entries();

    let mapping: [bigint, Uint8Array][] = assetMapping
      .map(([key, value]) => [BigInt(key.args[0].toString()), value.unwrap().asEthereum.toU8a()]);
    const idToL1Asset = new Map<bigint, Uint8Array>(mapping);

    const balances = await this.api.query.tokens.accounts.entries(address);
    const values: [Uint8Array, bigint][] = balances
      .filter(([key, _value]) => idToL1Asset.has(BigInt(key.args[1].toString())))
      .map(([key, value]) => {
        let tokenAddress = idToL1Asset.get(BigInt(key.args[1].toString()))!;
        return [tokenAddress, BigInt(value.free.toString())];
      });

    return new Map<Uint8Array, bigint>(values);
  }

  ferryDeposits(deposit: object): Promise<boolean> {
    throw new Error("Method not implemented.");
  }

  async getLastProcessedRequestId(): Promise<bigint> {
    const last = await this.api.query.rolldown.lastProcessedRequestOnL2(L1_CHAIN);
    return BigInt(last.toString());
  }

  async isExecuted(depositId: bigint): Promise<boolean> {
    return depositId <= (await this.getLastProcessedRequestId());
  }

  async isFerried(depositHash: Uint8Array): Promise<boolean> {
    if (depositHash.length !== 32) {
      throw new Error("depositHash must be exactly 32 bytes long.");
    }
    let chain = getL1ChainType(this.api);
    let status = await this.api.query.rolldown.ferriedDeposits([chain, depositHash]);
    return status.isSome;
  }


}


function getL1ChainType(api: ApiPromise): PalletRolldownMessagesChain {
  return api.createType("PalletRolldownMessagesChain", L1_CHAIN);
}

function sleep(timeInMilliseconds: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, timeInMilliseconds));
}

function getKeyring(type: KeypairType): Keyring {
  return new Keyring({ type });
}

function getCollator(type: KeypairType, mnemonic: string): KeyringPair {
  const keyring = getKeyring(type);
  return keyring.addFromSeed(hexToU8a(mnemonic));
}

async function getApi(nodeUrl: string): Promise<ApiPromise> {
  const api = await Mangata.instance([nodeUrl]).api();
  await api.isReady;
  return api;
}


async function getNativeL1Update(
  api: ApiPromise,
  encodedData: `0x${string}`,
): Promise<Option<PalletRolldownMessagesL1Update>> {
  return await api.rpc.rolldown.get_native_sequencer_update(
    encodedData.substring(2),
  );
}

async function getEvents(
  apiAt: ApiDecoration<"promise">,
  section: string,
  method: string,
): Promise<FrameSystemEventRecord[]> {
  const events = await apiAt.query.system.events();

  return events.filter(
    (event) => event.event.section === section && event.event.method === method,
  );
}

function isSuccess(events: MangataGenericEvent[]) {
  return events.some(
    (event) =>
      event.section === "system" && event.method === "ExtrinsicSuccess",
  );
}


function print(data: any) {
  console.log(util.inspect(data, { depth: null }));
}


export {
  print,
  sleep,
  getApi,
  getEvents,
  isSuccess,
  getCollator,
  getNativeL1Update,
  L1Api,
  L1Interface,
  L2Api,
  L2Interface,
  dummyDeposit,
  Ferry,
  Deposit,
};
