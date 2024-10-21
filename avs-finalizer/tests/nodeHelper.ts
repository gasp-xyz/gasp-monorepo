import {Keyring} from "@polkadot/api";
import {DockerUtils} from "./DockerUtils";
import {Mangata, signTx} from "gasp-sdk";

export async function getRpcPendingUpdateHash(_api : any, _blockHash: string) {
    throw new Error("Not implemented");
    // const result = await api.rpc.rolldown.pending_l2_requests_hash("Ethereum", blockHash);
    // return result.toHuman();
}
export async function createAWithdrawWithManualBatch(chain = "Ethereum", num = 1) {
    const api = await buildApi();
    const keyring = new Keyring({type: "ethereum"});
    const alice = keyring.addFromUri("0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133");
    keyring.addPair(alice);
    const tx = api.tx.utility.batchAll(
        [
            api.tx.rolldown.withdraw(
                api.createType("PalletRolldownMessagesChain", chain),
                alice.address,
                "0xfd471836031dc5108809d173a067e8486b9047a3",
                11,
                null),
            api.tx.rolldown.createBatch(
                api.createType("PalletRolldownMessagesChain", chain),
                null)
        ]
    );
    for (let i = 0; i < num; i++) {
        await signTx(api, tx, alice);
    }

}
export async function buildApi() {
    const mangata = Mangata.instance([new DockerUtils().bigStakeLocalEnvironment.SUBSTRATE_RPC_URL]);
    return await mangata.api();
}


