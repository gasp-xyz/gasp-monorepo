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
    let token;
    if(chain === "Ethereum"){
        const gaspToken = await api.query.assetRegistry.idToL1Asset(0);
        token = JSON.parse(JSON.stringify(gaspToken));
    }
    //@ts-ignore
    const gaspAddr = token.ethereum;
    const tx = api.tx.utility.batchAll(
        [
            api.tx.rolldown.withdraw(
                api.createType("PalletRolldownMessagesChain", chain),
                alice.address,
                gaspAddr,
                11,
                null),
            api.tx.rolldown.createBatch(
                api.createType("PalletRolldownMessagesChain", chain),
                null)
        ]
    );
    for (let i = 0; i < num; i++) {
        console.log("Batching updates!... ")
        const events = await signTx(api, tx, alice);
        console.log(JSON.stringify(events));
    }

}
export async function buildApi() {
    const mangata = Mangata.instance([new DockerUtils().bigStakeLocalEnvironment.SUBSTRATE_RPC_URL]);
    return await mangata.api();
}


