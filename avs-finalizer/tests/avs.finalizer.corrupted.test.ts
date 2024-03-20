import { jest, describe, it, afterEach , expect } from "@jest/globals";
import {DockerUtils} from "./DockerUtils";
import {
    createPublicClient,
    webSocket,
} from "viem";

import {anvil} from "./ethUtils";
import {
    getOperatorId,
    optOut,
    waitForNoTaskResponded,
    waitForOperatorRegistered,
} from "./operatorUtilities";
jest.setTimeout(1500000);

let dockerUtils: DockerUtils;


describe('Corrupted AVS Finalizer', () => {
    it('It Responds to tasks, but wont be considered - avs wont store any response - 2 mins', async () => {
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
        console.info("Corrupted - Operator Address: " + POperatorAddress + " Id " + operatorId);

        const noResponse = await waitForNoTaskResponded(publicClient, 120);
        expect(noResponse).toBe(true);

    });
    afterEach(async () => {
        await optOut(dockerUtils);
        await dockerUtils.stopContainer();
    });
});

