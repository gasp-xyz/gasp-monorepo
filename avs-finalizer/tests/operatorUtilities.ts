import {PublicClient} from "viem";

// @ts-ignore
import registryCoordinator from "./abis/RegistryCoordinator.json";

// @ts-ignore
import finalizerTaskManager from "./abis/FinalizerTaskManager.json";
import {DockerUtils} from "./DockerUtils";
export const registryCoordinatorAddress = '0x851356ae760d987E095750cCeb3bC6014560891C'
export const taskManagerAddress = "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8";

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
    return waitFor(publicClient, numTasks, "TaskResponded");
}

export async function waitFor(publicClient: PublicClient, numTasks = 1, eventName="TaskResponded") : Promise<any[]> {
    let tasks : any[] = [];
    console.info("Waiting for :" + numTasks + " tasks to be responded to..");
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


export async function waitForNo(publicClient: PublicClient, waitingTime = 120 , eventName="TaskResponded") : Promise<boolean> {
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
// @ts-ignore
BigInt.prototype.toJSON = function() { return this.toString() }