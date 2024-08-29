import {afterEach, describe, expect, it, jest} from "@jest/globals";
import {DockerUtils} from "./DockerUtils";
import {createPublicClient, defineChain, webSocket,} from "viem";

// @ts-ignore
import registryCoordinator from "../../contracts/out/RegistryCoordinator.sol/RegistryCoordinator.json";
import {toIncludeAllMembers} from "jest-extended";

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
expect.extend( { toIncludeAllMembers} );
import 'jest-extended';


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
let secContainer: DockerUtils;
let thirdContainer: DockerUtils;

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
    it('operator that does not respond -> eject -> rejoin ( <10b ) -> rejoin ( > 10b )', async () => {
        dockerUtils = new DockerUtils();
        const transport = webSocket("ws://0.0.0.0:8545" , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil3,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        const { edcsa , bls } =  await dockerUtils.startContainer();
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
        console.log("Re-Register STEP: operatorAddress: " + operatorAddress);

        //re-register
        secContainer = new DockerUtils();
        const errMessage = "Contract call reverted with message: ServiceManager.registerOperatorToAVS: minimum blocks elapsed limit for recurrent registration not met";
        await secContainer.startContainer(undefined, undefined, { edcsa , bls } , errMessage );
        //validate still in opt-out
        const statusAfterReRegister = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        expect(statusAfterReRegister).toBe(2);
        await validateOperatorOptOutStakeRegistry(publicClient, operatorAddress as string);
        await validateOperatorOptOutIndexRegistry(publicClient, operatorAddress as string, statusBeforeOptOut);

        await secContainer.stopContainer();

        await mineEthBlocks(10);
        console.log("After-Waiting and Re-Register STEP: operatorAddress: " + operatorAddress);

        //re-register again
        thirdContainer = new DockerUtils();
        await thirdContainer.startContainer(undefined, undefined, { edcsa , bls });
        const statusAfterWaitingAndReJoined = await publicClient.readContract({
            address: registryCoordinatorAddress,
            abi: registryCoordinator.abi,
            functionName: "getOperatorStatus",
            args: [operatorAddress],
        });
        console.log("Validate status - OK " + operatorAddress);
        expect(statusAfterWaitingAndReJoined).toBe(1);
        await validateOperatorOptInStakeRegistry(publicClient, operatorAddress as string);
        await validateOperatorOptInIndexRegistry(publicClient, operatorAddress as string);
        await thirdContainer.stopContainer();
        //we need to wait for it being de-registered
        await waitForOperatorDeRegistered(publicClient);

    });
    afterEach(async () => {
        //try opt-out just in case.
        try {
            await dockerUtils.container?.exec("./main opt-out-avs").then((result) => {
                console.log(result);
            }).catch((err) => {
                console.error(err);
            });
            await dockerUtils.stopContainer();
            await secContainer.container?.exec("./main opt-out-avs").then((result) => {
                console.log(result);
            }).catch((err) => {
                console.error(err);
            });
            await secContainer.stopContainer();
            await thirdContainer.container?.exec("./main opt-out-avs").then((result) => {
                console.log(result);
            }).catch((err) => {
                console.error(err);
            });
            await thirdContainer.stopContainer();
        }catch (e) {
            console.info("Opt-out failed, probably the container is not running");
        }

    });

});

describe("AVS Finalizer - tasks", () => {
    it('When operator online -> threshold changes && task response is submitted', async () => {
        dockerUtils = new DockerUtils();
        const publicClient = getPubClient();

        //let's wait for 5 tasks to avoid quorum numbers from other tests.
        const taskBefore = await waitForTaskResponded(publicClient, 5).then((tasks) => {
            return tasks.map( x=> x.args.taskResponseMetadata)
        })
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer(dockerUtils.FINALIZER_IMAGE, dockerUtils.bigStakeLocalEnvironment);
        const operatorAddress = await POperatorAddress;
        console.log("operatorAddress: " + operatorAddress);
        const taskAfter = await waitForTaskResponded(publicClient, 4).then((tasks) => {
            expect(tasks).toHaveLength(4);
            return tasks.map( x=> x.args.taskResponseMetadata)
        })
        expect(taskBefore).not.toEqual(taskAfter);
        const quorumBefore = BigInt(taskBefore[taskBefore.length -1].quroumStakeTotals[0]) ;
        //used the latest task  event to avoid flakiness [3]
        const quorumAfter = BigInt(taskAfter[taskAfter.length -1].quroumStakeTotals[0]);
        const operatorStake = BigInt(dockerUtils.bigStakeLocalEnvironment.STAKE);
        expect(quorumAfter - quorumBefore).toBe(operatorStake);
        const pTaskCompleted = waitFor(publicClient, 1, "TaskCompleted");
        const taskRespondedWithOp = await waitForTaskResponded(publicClient, 1).then((tasks) => {
            return tasks.map( x=> JSON.parse(JSON.stringify(x)))
        });
        const taskCompleted = await pTaskCompleted;
        expect(taskCompleted).toHaveLength(1);
        expect(taskCompleted[0].args.taskIndex).toBe(taskRespondedWithOp[0].args.taskResponse.referenceTaskIndex);
        console.log(taskRespondedWithOp)
        await validateTaskDataFromEvent(
            publicClient,
            taskRespondedWithOp[0].args.taskResponse.referenceTaskIndex,
            taskRespondedWithOp[0].args.taskResponse,
            BigInt(taskRespondedWithOp[0].blockNumber),
            taskRespondedWithOp[0].transactionHash );
        //opt-out
        await dockerUtils.container?.exec("./main opt-out-avs").then((result) => {
            console.log(result);
        }).catch((err) => {
            console.error(err);
        });
        const pTaskCompletedAfterOptOut = waitFor(publicClient, 3, "TaskCompleted");
        await mineEthBlocks(5);
        const taskAfterOptOut = await waitForTaskResponded(publicClient, 3).then((tasks) => {
            return tasks.map( x=> JSON.parse(JSON.stringify(x)))
        });
        const taskCompletedAfterOptOut = await pTaskCompletedAfterOptOut;
        // let's wait for 3 tasks, it can happen that quorum is not updated in the first task
        const quorumAfterOptOut = BigInt(taskAfterOptOut[2].args.taskResponseMetadata.quroumStakeTotals[0]);
        expect(quorumAfterOptOut).toBe(quorumBefore);

        //Quorum must be adapted, so it can happen that some tasks are not completed, but at least one must be.
        //@ts-ignore
        expect(taskCompletedAfterOptOut.flatMap( x=> x.args.taskIndex)).toIncludeAllMembers(taskAfterOptOut.flatMap( x=> x.args.taskResponse.referenceTaskIndex));
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
