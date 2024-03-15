import { jest, describe, it, expect, afterEach } from "@jest/globals";
import { DockerUtils } from "./DockerUtils";
import {
    createPublicClient, defineChain, PublicClient,
    webSocket,
} from "viem";

// @ts-ignore
import registryCoordinator from "./abis/RegistryCoordinator.json";
const registryCoordinatorAddress = '0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9'


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
            http: ["ws://0.0.0.0:8545"],
        },
        default: {
            http: ["ws://0.0.0.0:8545"],
        },
    },
});
let dockerUtils: DockerUtils;

describe('AVS Finalizer', () => {
    it('opt-out', async () => {
        dockerUtils = new DockerUtils();
        const transport = webSocket("ws://0.0.0.0:8545" , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil3,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer();
        const operatorAddress = await POperatorAddress;
        console.log("operatorAddress: " + operatorAddress);
        const res = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        expect(res).toBe(1);

        const PoperatorDeregisteredAddress = waitForOperatorDeRegistered(publicClient);

        // opt-out
        await dockerUtils.container?.exec("./main opt-out-avs").then((result) => {
            console.log(result);
        }).catch((err) => {
            console.error(err);
        });
        const deRegistered = await PoperatorDeregisteredAddress;
        expect(deRegistered).toBe(operatorAddress);

        const statusAfter = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        expect(statusAfter).toBe(2);

    });
    it('eject', async () => {
        dockerUtils = new DockerUtils();
        const transport = webSocket("ws://0.0.0.0:8545" , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil3,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer();
        const operatorAddress = await POperatorAddress;
        console.log("operatorAddress: " + operatorAddress);
        const res = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        expect(res).toBe(1);
        await dockerUtils.stopContainer();

        const PoperatorDeregisteredAddress = waitForOperatorDeRegistered(publicClient);


        const deRegistered = await PoperatorDeregisteredAddress;
        expect(deRegistered).toBe(operatorAddress);

        const statusAfter = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        expect(statusAfter).toBe(2);

    });
    afterEach(async () => {
        await dockerUtils.stopContainer();
    });
});

function waitForOperatorRegistered(publicClient: PublicClient) {
    return new Promise((resolve, _) => {
        publicClient.watchEvent({
            address: registryCoordinatorAddress,
            event: {
                type: "event",
                name: "OperatorRegistered",
                inputs: [
                    {name: "operator", type: "address", indexed: true, internalType: "address"},
                    {name: "operatorId", type: "bytes32", indexed: true, internalType: "bytes32"}],
                anonymous: false
            },
            onLogs: async (logs) => {
                for (const log of logs) {
                    const operator = log.args.operator;
                    console.debug(JSON.stringify(operator));
                    resolve(operator);
                }
            },
        });
    })
}
function waitForOperatorDeRegistered(publicClient: PublicClient) {
    return new Promise((resolve, _) => {
        publicClient.watchEvent({
            address: registryCoordinatorAddress,
            event: {
                "type":"event",
                "name":"OperatorDeregistered",
                "inputs":[
                    {"name":"operator","type":"address","indexed":true,"internalType":"address"},
                    {"name":"operatorId","type":"bytes32","indexed":true,"internalType":"bytes32"}],
                "anonymous":false
            },
            onLogs: async (logs) => {
                for (const log of logs) {
                    const operator = log.args.operator;
                    console.debug("Deregistered"  + JSON.stringify(operator));
                    resolve(operator);
                }
            },
        });
    })
}