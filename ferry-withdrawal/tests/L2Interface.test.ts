import { describe, expect, it } from "vitest";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { getApi , L2Api , getL1ChainType} from "../src/l2/L2Api.js";
import { signTx } from "gasp-sdk";
import 'dotenv/config'

const URI = "ws://localhost:9944";
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


    it('should fetch merkle proof', async () => {
      let api = await getApi(URI);
      let l2 = new L2Api(api);
      const nativeTokenAddress = await l2.getNativeTokenAddress();
      await dummyWithdrawal(api, nativeTokenAddress, 1n, 0n);
      await dummyWithdrawal(api, nativeTokenAddress, 1n, 0n);
      const result = await l2.getMerkleProof(1n, 2n, 2n);
      expect(result.length).to.be.greaterThan(0);
    }, {timeout: 1200000});

});
