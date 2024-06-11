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


describe('Corrupted AVS Finalizer', () => {
    it('It Responds to tasks, but wont be considered - avs wont store any Completeness - 2 mins', async () => {
        console.info("Starting Corrupted Finalizer");
        dockerUtils = new DockerUtils();
        const transport = webSocket(anvil.rpcUrls.default.http[0] , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer(dockerUtils.FINALIZER_IMAGE , dockerUtils.corruptedFinalizerLocalEnvironment);
        const address = await POperatorAddress;
        console.info("Started");
        const operatorId = await getOperatorId(publicClient, address as string);
        // lets wait for some time, to be sure that the operator fully onboard.
        await new Promise(r => setTimeout(r, 10000));

        console.info("Corrupted - Operator Address: " + POperatorAddress + " Id " + operatorId);
        const noCompleted = waitForNo(publicClient, 120, "TaskCompleted");
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

describe('Non Corrupted AVS Finalizer', () => {
    it('It Responds to tasks, and completeness - 2 mins', async () => {
        console.info("Starting Non-Corrupted Finalizer");
        dockerUtils = new DockerUtils();
        const transport = webSocket(anvil.rpcUrls.default.http[0] , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil,
        });
        const POperatorAddress = waitForOperatorRegistered(publicClient);
        await dockerUtils.startContainer(dockerUtils.FINALIZER_IMAGE , dockerUtils.finalizerLocalEnvironment);
        const address = await POperatorAddress;
        console.info("Started");
        const operatorId = await getOperatorId(publicClient, address as string);
        await validateBLSApkRegistry(publicClient, address as string, operatorId);
        // lets wait for some time, to be sure that the operator fully onboard.
        await new Promise(r => setTimeout(r, 10000));

        console.info("Operator Address: " + POperatorAddress + " Id " + operatorId);
        const completed = waitFor(publicClient, 1, "TaskCompleted");
        const responded = waitForTaskResponded(publicClient, 1);
        const [response, completedResponse] = await Promise.all([responded, completed]);
        expect(response.length).toBeGreaterThan(0);
        expect(completedResponse.length).toBeGreaterThan(0);

    });
    afterEach(async () => {
        await optOut(dockerUtils);
        await dockerUtils.stopContainer();
    });
});


