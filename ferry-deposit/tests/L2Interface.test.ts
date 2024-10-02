import { describe, test, beforeAll, expect, it } from "vitest";
import { getApi } from "../src/utils/index.js";
import { L2Interface, L2Api } from "../src/l2";
import { hexToU8a } from "@polkadot/util";
const timeout = 60000;

const URI = "ws://localhost:9944";
const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";

describe('L2Interface', () => {
    it('should successfully connect through websocket', async () => {
      let api = await getApi(URI);
    });

    it('should fetch balances from local chain', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      let balances = await l2.getBalances(hexToU8a(ALITH));
      expect(balances.size).toBeGreaterThan(0);
      for (let [_key, value] of balances) {
        expect(value).toBeGreaterThan(0);
      }
    });

    it('can query last processed request id', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      await l2.getLastProcessedRequestId();
    });

    it('can check if deposits was executed', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      const lastExecuted = await l2.getLastProcessedRequestId();
      expect(await l2.isExecuted(lastExecuted)).toBeTruthy();
      expect(await l2.isExecuted(lastExecuted + 1n)).toBeFalsy();
    });

    it('can check if deposits was ferried', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      expect(await l2.isFerried(new Uint8Array(32))).toBeFalsy();
    });

    it('can fetch native asset id', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      await l2.getNativeTokenAddress();
    });

    it('can valuate token', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      await l2.valutateToken(hexToU8a("0000000000000000000000000000000000000001"), 10n);

      let nativeTokenAddress = await l2.getNativeTokenAddress();
      expect(await l2.valutateToken(nativeTokenAddress, 10n)).to.be.equal(10n);
    });

    // TODO: add in the end
    it('consecutive ferry will fail', async () => {
    });


});
