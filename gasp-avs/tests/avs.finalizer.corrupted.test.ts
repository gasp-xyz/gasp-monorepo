import { jest, describe, it, afterEach , expect } from "@jest/globals";
import {DockerUtils} from "./DockerUtils";
import {
    createPublicClient,
    webSocket,
} from "viem";

import {anvil} from "./ethUtils";
import {
    getOperatorId,
    optOut, waitFor, waitForNo,
    waitForOperatorRegistered, waitForTaskResponded,
} from "./operatorUtilities";
import {validateBLSApkRegistry} from "./validators";
jest.setTimeout(1500000);

let dockerUtils: DockerUtils;

//TODO: Unskip when syncer is developed and in place.
describe.skip('Corrupted Gasp AVS', () => {
    it.skip('It Responds to tasks, but wont be considered - avs wont store any Completeness - 2 mins', async () => {
        console.info("Starting Corrupted Gasp AVS");
        dockerUtils = new DockerUtils();
        const transport = webSocket(anvil.rpcUrls.default.http[0] , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer(dockerUtils.GASP_AVS_IMAGE , dockerUtils.corruptedFinalizerLocalEnvironment);
        const address = await POperatorAddress;
        console.info("Started");
        const operatorId = await getOperatorId(publicClient, address as string);
        // lets wait for some time, to be sure that the operator fully onboard.
        await new Promise(r => setTimeout(r, 10000));

        console.info("Corrupted - Operator Address: " + POperatorAddress + " Id " + operatorId);
        const noCompleted = waitForNo(publicClient, 120, "RdTaskCompleted");
        const responded = waitForTaskResponded(publicClient, 1);
        const [response, isNoCompleted] = await Promise.all([responded, noCompleted]);
        expect(response.length).toBeGreaterThan(0);
        expect(isNoCompleted).toBe(true);

    });
    afterEach(async () => {
        await optOut(dockerUtils);
        await dockerUtils.stopContainer();
    });
});

describe.skip('Non Corrupted Gasp AVS', () => {
    it.skip('It Responds to tasks, and completeness - 2 mins', async () => {
        console.info("Starting Non-Corrupted Gasp AVS");
        dockerUtils = new DockerUtils();
        const transport = webSocket(anvil.rpcUrls.default.http[0] , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        const completedBefore = await waitFor(publicClient, 1, "RdTaskCompleted");
        await dockerUtils.startContainer(dockerUtils.GASP_AVS_IMAGE , dockerUtils.finalizerLocalEnvironment);
        const address = await POperatorAddress;
        console.info("Started");
        await waitFor(publicClient, 1, "OpTaskCompleted");
        const operatorId = await getOperatorId(publicClient, address as string);
        await validateBLSApkRegistry(publicClient, address as string, operatorId);
        // lets wait for some time, to be sure that the operator fully onboard.
        await new Promise(r => setTimeout(r, 10000));

        console.info("Operator Address: " + POperatorAddress + " Id " + operatorId);
        const completed = waitFor(publicClient, 1, "RdTaskCompleted");
        const responded = waitForTaskResponded(publicClient, 1);
        const [response, completedResponse] = await Promise.all([responded, completed]);
        expect(response.length).toBeGreaterThan(0);
        expect(completedResponse.length).toBeGreaterThan(0);
        const quorumBefore = BigInt(completedBefore[completedBefore.length -1].quroumStakeTotals[0]) ;
        //used the latest task  event to avoid flakiness [3]
        const quorumAfter = BigInt(completedResponse[completedResponse.length -1].quroumStakeTotals[0]);
        expect(quorumAfter).toBeGreaterThan(quorumBefore);
    });
    afterEach(async () => {
        await optOut(dockerUtils);
        await dockerUtils.stopContainer();
    });
});


