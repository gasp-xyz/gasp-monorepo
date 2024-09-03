import {PublicClient} from "viem";

// @ts-ignore
import registryCoordinator from "../../contracts/out/RegistryCoordinator.sol/RegistryCoordinator.json";
// @ts-ignore
import finalizerTaskManager from "../../contracts/out/FinalizerTaskManager.sol/FinalizerTaskManager.json";
// @ts-ignore
import blsApkRegistry from "../../contracts/out/BLSApkRegistryStorage.sol/BLSApkRegistryStorage.json";
// @ts-ignore
import stakeRegistry from "../../contracts/out/StakeRegistryStorage.sol/StakeRegistryStorage.json";
// @ts-ignore
import indexRegistry from "../../contracts/out/IndexRegistryStorage.sol/IndexRegistryStorage.json";
//@ts-ignore
import deploymentJson from "../../contracts/script/output/31337/avs_deployment_output.json";

import {DockerUtils} from "./DockerUtils";
//IF those addresses gets de-sync: call the makefile deploy all save state, to update it , and commit the changes
export const registryCoordinatorAddress : `0x${string}` = deploymentJson.addresses.registryCoordinator as `0x${string}`;
export const taskManagerAddress : `0x${string}` = deploymentJson.addresses.taskManager as `0x${string}`;
export const blsApkRegistryAddress : `0x${string}` = deploymentJson.addresses.blsApkRegistry as `0x${string}`;
export const stakeRegistryAddress : `0x${string}` = deploymentJson.addresses.stakeRegistry as `0x${string}`;
export const indexRegistryAddress : `0x${string}` = deploymentJson.addresses.indexRegistry as `0x${string}`;

export const DEFAULT_QUORUM = 0;

export async function waitForOperatorRegistered(publicClient: PublicClient) {
    return new Promise((resolve, _) => {
        const unwatch = publicClient.watchEvent({
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
                    unwatch()
                    resolve(operator);
                }
            },
        });
    })
}
export async function waitForOperatorDeRegistered(publicClient: PublicClient) {
    return new Promise((resolve, _) => {
        const unwatch = publicClient.watchEvent({
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
                    unwatch();
                    resolve(operator);
                }
            },
        });
    })
}

export async function getOperatorId(publicClient: any, operatorAddress: string) {
    const res = await publicClient.readContract({
        address: registryCoordinatorAddress,
        abi: registryCoordinator.abi,
        functionName: "getOperatorId",
        args: [operatorAddress],
    });
    return res as any as string;
}

export async function waitForTaskResponded(publicClient: PublicClient, numTasks = 1) : Promise<any[]> {
    return waitFor(publicClient, numTasks, "RdTaskResponded");
}

export async function waitFor(publicClient: PublicClient, numTasks = 1, eventName="RdTaskResponded") : Promise<any[]> {
    let tasks : any[] = [];
    console.info("Waiting for :" + numTasks + " tasks to be " + eventName + "..");
    return await  new Promise( (resolve) => {
        const unwatch = publicClient.watchContractEvent({
            abi : finalizerTaskManager.abi,
            address: taskManagerAddress,
            eventName: eventName,
            onLogs: async (logs) => {
                if(tasks.length < numTasks) {
                    tasks = tasks.concat(logs);
                } else {
                    unwatch();
                    resolve(tasks);
                }

            },
        });
    } )
}


export async function waitForNo(publicClient: PublicClient, waitingTime = 120 , eventName="RdTaskResponded") : Promise<boolean> {
    console.info("Waiting for :" + waitingTime + " secs to be " + eventName + "..");
    return await  new Promise( (resolve) => {
        const unwatch = publicClient.watchContractEvent({
            abi : finalizerTaskManager.abi,
            address: taskManagerAddress,
            eventName: eventName,
            onLogs: async (logs) => {
                console.warn("Oh oh , got an event : " + JSON.stringify(logs));
                resolve(false);
            },
        });
        setTimeout( () => {
            unwatch();
            resolve(true);
        }, waitingTime * 1000);
    } )
}


export async function optOut(dockerUtilsInstance: DockerUtils){
    await dockerUtilsInstance.container?.exec("./main opt-out-avs").then((result) => {
        console.log(result);
    }).catch((err) => {
        console.error(err);
    });
}

export async function getEntryFromBlsApkRegistry(publicClient: PublicClient, functionName: string, args: any[]) {
    const res = await publicClient.readContract({
        address: blsApkRegistryAddress,
        abi: blsApkRegistry.abi,
        functionName: functionName,
        args: args,
    });
    return res as any as string[];
}

export async function getEntryFromStakeRegistry(publicClient: PublicClient, functionName: string, args: any[]) {
    const res = await publicClient.readContract({
        address: stakeRegistryAddress,
        abi: stakeRegistry.abi,
        functionName: functionName,
        args: args,
    });
    return res as any as string[];
}


export async function getEntryFromIndexRegistry(publicClient: PublicClient, functionName: string, args: any[]) {
    const res = await publicClient.readContract({
        address: indexRegistryAddress,
        abi: indexRegistry.abi,
        functionName: functionName,
        args: args,
    });
    return res as any as string[];
}

export async function getEntryFromTaskManagerRegistry(publicClient: PublicClient, functionName: string, args: any[]) {
    const res = await publicClient.readContract({
        address: taskManagerAddress,
        abi: finalizerTaskManager.abi,
        functionName: functionName,
        args: args,
    });
    return res as any as string[];
}


// @ts-ignore
BigInt.prototype.toJSON = function() { return this.toString() }