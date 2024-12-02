import {describe, expect, it, jest} from "@jest/globals";
import {DockerUtils, getNewKeys} from "./DockerUtils";

// @ts-ignore
import request from "supertest";
import {createPublicClient, webSocket} from "viem";
import {anvil} from "./ethUtils";
import {waitFor, waitForOperatorRegistered} from "./operatorUtilities";

jest.setTimeout(1500000);

describe('AVS Aggregator', () => {
    it('Rpc test - validate format', async () => {
        const keys = await getNewKeys();
        console.info("keys: " + JSON.stringify(keys));
        const json = {
            "TaskResponse":
                {
                    "ReferenceTaskIndex": 1,
                    "BlockHash": [153, 125, 3, 162, 144, 139, 84, 185, 75, 251, 230, 179, 126, 94, 134, 245, 138, 35, 211, 101, 51, 39, 202, 53, 238, 48, 25, 161, 103, 238, 121, 135],
                    "StorageProofHash": [45, 198, 26, 130, 242, 237, 121, 120, 211, 137, 249, 188, 50, 113, 112, 21, 154, 175, 48, 174, 130, 254, 201, 183, 204, 87, 100, 164, 51, 116, 215, 243],
                    "PendingStateHash": [31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25]
                },
            "BlsSignature": {
                "g1_point": {
                    "X": "19808572461450981585557502876389973434454437096509251305252587730583512629969",
                    "Y": "19308770365116519225542267109925445620022708019098048034973350536302967625092"
                }
            },
            "OperatorId": [236, 36, 44, 60, 118, 118, 149, 182, 164, 57, 177, 4, 122, 222, 40, 96, 148, 224, 15, 95, 21, 182, 114, 38, 125, 245, 135, 139, 154, 157, 15, 199]
        }

        const response = await request("http://localhost:8090").post('/').send(json);
        expect(response.status).toBe(404);
        expect(response.text).toEqual("task 1 not initialized or already completed\n");
    });
    it('Rpc test - signature', async () => {
        const dockerUtils = new DockerUtils();
        const transport = webSocket(anvil.rpcUrls.default.http[0] , {
            retryCount: 5,
        });
        const publicClient = createPublicClient({
            transport,
            chain: anvil,
        });
        // opt-in to avoid avs completeness.
        const POperatorAddress = waitForOperatorRegistered(publicClient as any);
        await dockerUtils.startContainer(dockerUtils.FINALIZER_IMAGE, dockerUtils.bigStakeWithWrongAvsPort);
        await POperatorAddress;

        const keys = await getNewKeys();
        console.info("keys: " + JSON.stringify(keys));

        const currTask = await waitFor(publicClient as any, 1, "NewTaskCreated");

        const {taskIndex} = currTask[0].args;
        const json = {
            "TaskResponse":
                {
                    "ReferenceTaskIndex": taskIndex,
                    "BlockHash": [153, 125, 3, 162, 144, 139, 84, 185, 75, 251, 230, 179, 126, 94, 134, 245, 138, 35, 211, 101, 51, 39, 202, 53, 238, 48, 25, 161, 103, 238, 121, 135],
                    "StorageProofHash": [45, 198, 26, 130, 242, 237, 121, 120, 211, 137, 249, 188, 50, 113, 112, 21, 154, 175, 48, 174, 130, 254, 201, 183, 204, 87, 100, 164, 51, 116, 215, 243],
                    "PendingStateHash": [31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25]
                },
            "BlsSignature": {
                "g1_point": {
                    "X": "19808572461450981585557502876389973434454437096509251305252587730583512629969",
                    "Y": "19308770365116519225542267109925445620022708019098048034973350536302967625092"
                }
            },
            "OperatorId": [236, 36, 44, 60, 118, 118, 149, 182, 164, 57, 177, 4, 122, 222, 40, 96, 148, 224, 15, 95, 21, 182, 114, 38, 125, 245, 135, 139, 154, 157, 15, 199]
        }

        const response = await request("http://localhost:8090").post('/').send(json);
        await dockerUtils.container?.exec("./main opt-out-avs").then((result) => {
            console.log(result);
        }).catch((err) => {
            console.error(err);
        });
        expect(response.status).toBe(404);
        expect(response.text).toEqual(`task ${taskIndex} not initialized or already completed\n`);
    });

});
