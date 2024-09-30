import { describe, test, beforeAll, expect, it } from "vitest";
import { dummyDeposit, L1Api, L1Interface, L2Api, L2Interface, getApi } from "../src/utils/index.js";
import { hexToU8a } from "@polkadot/util";
import { TestClient, createPublicClient, createTestClient, webSocket } from "viem";
import util from "node:util";

const timeout = 60000;

const WS_URI = "ws://localhost:8545";
const HTTP_URI = "http://localhost:8545";
const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";
const ANVIL_TEST_ACCOUNT = "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";

const properImpl = (key: any, value: any) => {
  if (typeof value === 'bigint') {
    return value.toString();
  }
  return value;
};

async function mintBlocks() {
  const tc = createTestClient({
    mode: "anvil",
    transport: webSocket(WS_URI, { retryCount: 5 })
  });
  await tc.mine({ blocks: 100 })
}

describe('L2Interface', () => {
  beforeAll(async () => {
    /// Mint blocks(only on anvil) so the the contract deployment is reachable using viem 
    // 'blockTag' : 'latest'
    console.info('Minting blocks!!!!!');
    await mintBlocks();
    await dummyDeposit(WS_URI);
  });

  it('should successfully connect through websocket', async () => {
    let l1Api = new L1Api(WS_URI);
    expect(await l1Api.isRolldownDeployed()).toBeTruthy()
  });

  it('should successfully connect through http', async () => {
    let l1Api = new L1Api(HTTP_URI);
    expect(await l1Api.isRolldownDeployed()).toBeTruthy()
  });

  it('can fetch latestRequestId', async () => {
    await mintBlocks();
    let l1Api = new L1Api(HTTP_URI);
    let latestRequestId = await l1Api.getLatestRequestId();
    expect(latestRequestId).toBeGreaterThanOrEqual(0);
    await dummyDeposit(WS_URI);
    await mintBlocks();
    expect(await l1Api.getLatestRequestId()).toBeGreaterThan(latestRequestId!);
  });

  it('can fetch deposits', async () => {
    await mintBlocks();
    let l1Api = new L1Api(HTTP_URI);
    await dummyDeposit(WS_URI);
    await mintBlocks();
    let latestRequestId = await l1Api.getLatestRequestId();
    const deposits = await l1Api.getDeposits(1n, latestRequestId!);
    expect(deposits.length).toBeGreaterThan(0);
  });

  it('can fetch deposits hash', async () => {
    await mintBlocks();
    let l1Api = new L1Api(HTTP_URI);

    const firstId = (await l1Api.getLatestRequestId())!;
    await dummyDeposit(WS_URI);
    await mintBlocks();
    const secondId = (await l1Api.getLatestRequestId())!;
    expect(firstId).not.toEqual(secondId);

    const firstHash = await l1Api.getDepostiHash(firstId);
    const secondHash = await l1Api.getDepostiHash(secondId);
    expect(firstHash).not.toEqual(secondHash);

  });


});
