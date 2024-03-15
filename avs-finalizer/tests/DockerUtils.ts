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
        env.ECDSA_KEY_JSON = "{\"address\":\"860b6912c2d0337ef05bbc89b0c2cb6cbaeab4a5\",\"crypto\":{\"cipher\":\"aes-128-ctr\",\"ciphertext\":\"37ab29382354906d7b33b0ad1b00c539dfb69ea85997e470baba7601b0faa373\",\"cipherparams\":{\"iv\":\"b7642683ba782688cc4da76dd5c61fce\"},\"kdf\":\"scrypt\",\"kdfparams\":{\"dklen\":32,\"n\":262144,\"p\":1,\"r\":8,\"salt\":\"fdc108d3e6388174f4cf6d5abe6de2364c96dcc03c4f1bddb1b47b01dce742e2\"},\"mac\":\"d4fff8d963a7815a7203f8c73b3047a2e5efcefb8163e6df316cc55f79df0cc2\"},\"id\":\"0c401f60-9d1f-4918-a823-1866984f8fcd\",\"version\":3}"
        env.BLS_KEY_JSON = "{\"pubKey\":\"E([643552363890320897587044283125191574906281609959531590546948318138132520777,7028377728703212953187883551402495866059211864756496641401904395458852281995])\",\"crypto\":{\"cipher\":\"aes-128-ctr\",\"ciphertext\":\"8221f69293d98ca9d46c52a2747b61755c6bb71f41e951154af3bcbc0daef2b8\",\"cipherparams\":{\"iv\":\"d8a38c68f3400aeb93084672b6e3ae61\"},\"kdf\":\"scrypt\",\"kdfparams\":{\"dklen\":32,\"n\":262144,\"p\":1,\"r\":8,\"salt\":\"c41150482e6acfc2f0fcfd68b67f28ef22de8aa3f982048bdaf793c2e3ac3730\"},\"mac\":\"879c2831169d9ac70c662ebca90b9f3cd63761d0924bdbbde0a82a306baf44cd\"}}"
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
//        ECDSA_EPHEMERAL_KEY:"false" ,
//        BLS_EPHEMERAL_KEY:"false"
    }
}