import {ApiPromise, WsProvider} from "@polkadot/api";
import {DockerUtils} from "./DockerUtils";


export async function getRpcPendingUpdateHash(api : any, blockHash: string) {
    const result = await api.rpc.rolldown.pending_l2_requests_hash("Ethereum", blockHash);
    return result.toHuman();
}

export async function buildApi() {
    const wsProvider = new WsProvider(new DockerUtils().bigStakeLocalEnvironment.SUBSTRATE_RPC_URL);
    return await ApiPromise.create({
        provider: wsProvider,
        rpc: {
            rolldown:
                {
                    pending_updates_hash:
                        {
                            description: "Get the hash from the node",
                            params: [{
                                name: 'at',
                                type: 'Hash',
                                isOptional: true
                            }
                            ],
                            type: 'Vec<u8>'
                        },
                    pending_l2_requests_hash:
                        {
                            description: "Get the hash from the node",
                            params: [
                                {
                                    name: 'chain',
                                    type: 'String'
                                },
                                {
                                    name: 'at',
                                    type: 'Hash',
                                    isOptional: true
                                }
                            ],
                            type: 'Vec<u8>'
                        }
                },

        }
    });
}


