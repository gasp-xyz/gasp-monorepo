import { jest, describe, it } from "@jest/globals";
import { DockerUtils } from "./DockerUtils";
import {
    createPublicClient,
    webSocket,
} from "viem";

// import indexRegistry   from "./abis/IndexRegistryStorage.json" ;
// const indexRegistryAddress = "0x851356ae760d987E095750cCeb3bC6014560891C"

// import registryCoordinator   from "./abis/RegistryCoordinatorStorage.json" ;
// const registryCoordinatorAddress = "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9"

import registryCoordinator from "./abis/RegistryCoordinator.json";
import {defineChain} from "viem/utils/chain/defineChain";
const registryCoordinatorAddress = '0x8f86403A4DE0BB5791fa46B8e795C547942fE4Cf'


jest.setTimeout(1500000);

const anvil3 = defineChain({
    id: 31337,
    name: "anvil",
    network: "Anvil",
    nativeCurrency: {
        decimals: 18,
        name: "Ether",
        symbol: "ETH",
    },
    rpcUrls: {
        public: {
            http: ["ws://127.0.0.1:8545"],
        },
        default: {
            http: ["ws://127.0.0.1:8545"],
        },
    },
});

export const anvil2 = /*#__PURE__*/ defineChain({
    id: 31_337,
    name: 'Anvil',
    nativeCurrency: {
        decimals: 18,
        name: 'Ether',
        symbol: 'ETH',
    },
    rpcUrls: {
        default: {
            http: ['http://0.0.0.0:8545'],
            webSocket: ['ws://0.0.0.0:8545'],
        },
    },
})


describe('AVS Finalizer opt-out', () => {
    it('should opt-out of finalization', async () => {
        const dockerUtils = new DockerUtils();
        console.log("subscribing to eth events");
        const transport = webSocket("http://0.0.0.0:8545" , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            // @ts-ignore
            chain: anvil3,
        });
        const unwatch = publicClient.watchContractEvent({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            eventName: "OperatorRegistered",
            onLogs: async (logs) => {
                for (const log of logs) {
                    console.info(JSON.stringify(log));
                }
            },
        });
        await dockerUtils.startContainer();
        unwatch();
        await dockerUtils.stopContainer();
    });
});