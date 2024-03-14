import {GenericContainer, StartedTestContainer, Wait} from "testcontainers";
import {Environment} from "testcontainers/build/types";

export const FINALIZER_IMAGE_LOCAL = "mangatasolutions/avs-finalizer:local";
export const REDIS_HOST_DOCKER_IMAGE_NAME = "mangatasolutions/redis-test-stash:latest";
export const MAX_DAYS="max";
export const MAX_INTERVAL="day";

export class DockerUtils{
    container?: StartedTestContainer;
    containerName: string;
    constructor() {
        this.container = undefined;
        this.containerName = "";
    }
    async startContainer(image: string = FINALIZER_IMAGE_LOCAL, env = this.finalizerLocalEnvironment) {
        this.containerName = image;
        console.info("Starting container: " + image);
        if(!this.container){
            this.container = await new GenericContainer(image)
                .withWaitStrategy(Wait.forLogMessage("Testnet setup sucessfully, starting AVS verification"))
                .withEnvironment(env)
                .withNetworkMode("host")
                .start();

        }else{
            console.info("Container already started: " + this.container.getName());
        }
        return this.container;
    }
    async stopContainer() {
        console.info("Stopping container: " + this.containerName);
        await this.container!.stop();
    }

    finalizerLocalEnvironment : Environment = {
        RUST_LOG: "info",
        ETH_RPC_URL:"http://0.0.0.0:8545" ,
        ETH_WS_URL:"ws://0.0.0.0:8545" ,
        CHAIN_ID:"31337" ,
        SUBSTRATE_RPC_URL:"ws://0.0.0.0:9946" ,
        AVS_RPC_URL:"http://0.0.0.0:8090" ,
        AVS_REGISTRY_COORDINATOR_ADDR:"0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9" ,
        TESTNET:"true",
        ECDSA_EPHEMERAL_KEY:"true" ,
        BLS_EPHEMERAL_KEY:"true"
    }
}