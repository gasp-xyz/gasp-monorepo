import { describe, test, beforeAll, expect, it } from "vitest";
import { getApi } from "../src/utils/index.js";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import { L2Interface, L2Api , getL1ChainType} from "../src/l2";
import { hexToU8a } from "@polkadot/util";
import { Mangata, type MangataGenericEvent, signTx } from "gasp-sdk";
const timeout = 60000;

const URI = "ws://localhost:9944";
const NATIVE_TOKEN = "0x0000000000000000000000000000000000000001";
const ALITH_PRIVATE_KEY = "0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133";

async function dummyWithdrawal(api: ApiPromise, tokenAddress: Uint8Array, amount: bigint,ferryTip: bigint) :Promise<void> {
  const chain = getL1ChainType(api);
	const keyring = new Keyring({ type: "ethereum" });
	const keypair = keyring.createFromUri(ALITH_PRIVATE_KEY);

  const tx = api.tx.rolldown.withdraw(chain, keypair.address, tokenAddress, amount, ferryTip);
  await signTx(api, tx, keypair);
}

describe('L2Interface', () => {
    it('should successfully connect through websocket', async () => {
      let api = await getApi(URI);
    });

    it('should fetch native token address', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      const nativeTokenAddress = await l2.getNativeTokenAddress();
    });


    it('should fetch latest request id', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      const nativeTokenAddress = await l2.getNativeTokenAddress();

      let firstRequestId = await l2.getLatestRequestId();
      await dummyWithdrawal(api, nativeTokenAddress, 1n, 0n);

      let secondRequestId = await l2.getLatestRequestId();
      expect(firstRequestId).to.not.be.equal(secondRequestId);
    }, {timeout: 30000});


    it('should fetch withdrawals', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      const nativeTokenAddress = await l2.getNativeTokenAddress();

      let firstRequestId = await l2.getLatestRequestId();
      await dummyWithdrawal(api, nativeTokenAddress, 1n, 0n);
      let secondRequestId = await l2.getLatestRequestId();
      expect(secondRequestId).to.not.be.equal(null);

      const withdrawals = await l2.getWithdrawals(1n, secondRequestId!);
      expect(withdrawals.length).to.be.greaterThan(0);
    }, {timeout: 30000});

    //
    // it('can query last processed request id', async () => {
    //   let api = await getApi(URI);
    //   let l2 = new L2Api(api);
    //   await l2.getLastProcessedRequestId();
    // });
    //
    // it('can check if deposits was executed', async () => {
    //   let api = await getApi(URI);
    //   let l2 = new L2Api(api);
    //   const lastExecuted = await l2.getLastProcessedRequestId();
    //   expect(await l2.isExecuted(lastExecuted)).toBeTruthy();
    //   expect(await l2.isExecuted(lastExecuted + 1n)).toBeFalsy();
    // });
    //
    // it('can check if deposits was ferried', async () => {
    //   let api = await getApi(URI);
    //   let l2 = new L2Api(api);
    //   expect(await l2.isFerried(new Uint8Array(32))).toBeFalsy();
    // });
    //
    // it('can fetch native asset id', async () => {
    //   let api = await getApi(URI);
    //   let l2 = new L2Api(api);
    //   await l2.getNativeTokenAddress();
    // });
    //
    // it('can valuate token', async () => {
    //   let api = await getApi(URI);
    //   let l2 = new L2Api(api);
    //   await l2.valutateToken(hexToU8a("0000000000000000000000000000000000000001"), 10n);
    //
    //   let nativeTokenAddress = await l2.getNativeTokenAddress();
    //   expect(await l2.valutateToken(nativeTokenAddress, 10n)).to.be.equal(10n);
    // });
    //
    // // TODO: add in the end
    // it('consecutive ferry will fail', async () => {
    // });


});
