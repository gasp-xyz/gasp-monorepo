import {afterEach, describe, expect, it, jest} from "@jest/globals";
import {DockerUtils} from "./DockerUtils";
import {createPublicClient, defineChain, webSocket,} from "viem";

// @ts-ignore
import registryCoordinator from "./abis/RegistryCoordinator.json";
import {
    getOperatorId,
    registryCoordinatorAddress,
    waitFor,
    waitForOperatorDeRegistered,
    waitForOperatorRegistered,
    waitForTaskResponded
} from "./operatorUtilities";
import {
    getLatestQuorumUpdate,
    validateBLSApkRegistry,
    validateOperatorOptInIndexRegistry,
    validateOperatorOptInStakeRegistry,
    validateOperatorOptOutIndexRegistry,
    validateOperatorOptOutStakeRegistry,
    validateTaskDataFromEvent,
} from "./validators";


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

async function mineEthBlocks(blocks: number) {
    for (let i = 0; i < blocks; i++) {
        const host = anvil3.rpcUrls.default.http[0].replace("ws", "http");
        const myHeaders = new Headers();
        myHeaders.append("Content-Type", "application/json");
        const raw = JSON.stringify({
            method: "evm_mine",
            params: [],
            id: i,
            jsonrpc: "2.0",
        });
        const requestOptions = {
            method: "POST",
            headers: myHeaders,
            body: raw,
            redirect: "follow",
        };
        // @ts-ignore
        await fetch(host, requestOptions)
            .then((response) => response.text())
            .then((result) => console.log(result))
            .catch((error) => console.error(error));
    }

}

describe('AVS Finalizer', () => {
    it('opt-in / opt-out', async () => {
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

        await validateOperatorOptInStakeRegistry(publicClient, operatorAddress as string);
        await validateOperatorOptInIndexRegistry(publicClient, operatorAddress as string);
        const statusBeforeOptOut = await getLatestQuorumUpdate(publicClient);

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
        const tasks = await waitFor(publicClient, 2, "TaskCompleted");
         expect(tasks).toHaveLength(2);

         //Test that after op-out the operator still has the bls keys in the registry
        await validateBLSApkRegistry(publicClient, operatorAddress as string , await getOperatorId(publicClient, operatorAddress as string) as string);
        await validateOperatorOptOutStakeRegistry(publicClient, operatorAddress as string);
        await validateOperatorOptOutIndexRegistry(publicClient, operatorAddress as string, statusBeforeOptOut);

    });
    it('operator that does not respond -> eject', async () => {
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
        await mineEthBlocks(1);
        const statusBeforeOptOut = await getLatestQuorumUpdate(publicClient);

        const PoperatorDeregisteredAddress = waitForOperatorDeRegistered(publicClient);
        // 10s * 2 * 5 = 100s ( every two blocks we produce a task, and at 5th task we eject)
        const deRegistered = await PoperatorDeregisteredAddress;

        expect(deRegistered).toBe(operatorAddress);

        const statusAfter = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        expect(statusAfter).toBe(2);
        await validateOperatorOptOutStakeRegistry(publicClient, operatorAddress as string);
        await validateOperatorOptOutIndexRegistry(publicClient, operatorAddress as string, statusBeforeOptOut);

    });
    afterEach(async () => {
        await dockerUtils.stopContainer();
    });

});

describe("AVS Finalizer - tasks", () => {
    it('When operator online -> threshold changes && task response is submitted', async () => {
        dockerUtils = new DockerUtils();
        const publicClient = getPubClient();

        const taskBefore = await waitForTaskResponded(publicClient, 1).then((tasks) => {
            return tasks.map( x=> x.args.taskResponseMetadata)
        })

        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer(dockerUtils.FINALIZER_IMAGE, dockerUtils.bigStakeLocalEnvironment);
        const operatorAddress = await POperatorAddress;
        console.log("operatorAddress: " + operatorAddress);
        const taskAfter = await waitForTaskResponded(publicClient, 4).then((tasks) => {
            expect(tasks).toHaveLength(2);
            return tasks.map( x=> x.args.taskResponseMetadata)
        })
        expect(taskBefore).not.toEqual(taskAfter);
        const quorumBefore = BigInt(taskBefore[0].quroumStakeTotals[0]) ;
        //used the latest task  event to avoid flakiness [3]
        const quorumAfter = BigInt(taskAfter[3].quroumStakeTotals[0]);
        const operatorStake = BigInt(dockerUtils.bigStakeLocalEnvironment.STAKE);
        expect(quorumAfter - quorumBefore).toBe(operatorStake);

        const taskCompletedWithOperator = await waitForTaskResponded(publicClient, 1).then((tasks) => {
            return tasks.map( x=> JSON.parse(JSON.stringify(x)))
        })
        console.log(taskCompletedWithOperator)
        await validateTaskDataFromEvent(
            publicClient,
            taskCompletedWithOperator[0].args.taskResponse.referenceTaskIndex,
            taskCompletedWithOperator[0].args.taskResponse,
            BigInt(taskCompletedWithOperator[0].blockNumber),
            taskCompletedWithOperator[0].transactionHash );
        //opt-out
        await dockerUtils.container?.exec("./main opt-out-avs").then((result) => {
            console.log(result);
        }).catch((err) => {
            console.error(err);
        });
        await mineEthBlocks(5);
        const taskAfterOptOut = await waitForTaskResponded(publicClient, 3).then((tasks) => {
            return tasks.map( x=> x.args.taskResponseMetadata)
        });

        // let's wait for 3 tasks, it can happen that quorum is not updated in the first task
        const quorumAfterOptOut = BigInt(taskAfterOptOut[2].quroumStakeTotals[0]);
        expect(quorumAfterOptOut).toBe(quorumBefore);

    });
    it.skip('When operator is offline -> task non-responded are timedout until ejected', async () => {
        dockerUtils = new DockerUtils();
        const transport = webSocket("ws://0.0.0.0:8545" , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil3,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer(dockerUtils.FINALIZER_IMAGE, dockerUtils.bigStakeLocalEnvironment);
        const operatorAddress = await POperatorAddress;
        console.log("operatorAddress: " + operatorAddress);





    });
    afterEach(async () => {
        // opt-out
        await dockerUtils.container?.exec("./main opt-out-avs").then((result) => {
            console.log(result);
        }).catch((err) => {
            console.error(err);
        });
        await dockerUtils.stopContainer();
    });

    function getPubClient() {
        const transport = webSocket("ws://0.0.0.0:8545", {
            retryCount: 5,
        });
        return createPublicClient({
            transport,
            chain: anvil3,
        });
    }
});
