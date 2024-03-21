import {PublicClient} from "viem";

// @ts-ignore
import registryCoordinator from "./abis/RegistryCoordinator.json";

// @ts-ignore
import finalizerTaskManager from "./abis/FinalizerTaskManager.json";
import {DockerUtils} from "./DockerUtils";
export const registryCoordinatorAddress = '0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9'
export const taskManagerAddress = "0x9E545E3C0baAB3E08CdfD552C960A1050f373042";

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

export async function getOperatorId(publicClient: PublicClient, operatorAddress: string) {
    const res = await publicClient.readContract({
        address: registryCoordinatorAddress,
        abi: registryCoordinator.abi,
        functionName: "getOperatorId",
        args: [operatorAddress],
    });
    return res as any as string;
}

export async function waitForTaskResponded(publicClient: PublicClient, numTasks = 1) : Promise<any[]> {
    let tasks : any[] = [];
    console.info("Waiting for :" + numTasks + " tasks to be responded to..");
    return await  new Promise( (resolve) => {
        const unwatch = publicClient.watchContractEvent({
            abi : finalizerTaskManager.abi,
            address: taskManagerAddress,
            eventName: "TaskResponded",
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

export async function waitForNoTaskResponded(publicClient: PublicClient, waitingTime = 120) : Promise<boolean> {
    console.info("Waiting for :" + waitingTime + " tasks to be responded to..");
    return await  new Promise( (resolve) => {
        const unwatch = publicClient.watchContractEvent({
            abi : finalizerTaskManager.abi,
            address: taskManagerAddress,
            eventName: "TaskResponded",
            onLogs: async (logs) => {
                console.warn("Task responded: " + JSON.stringify(logs));
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
// @ts-ignore
BigInt.prototype.toJSON = function() { return this.toString() }