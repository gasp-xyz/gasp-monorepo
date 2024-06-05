import {GenericContainer, StartedTestContainer, Wait} from "testcontainers";
import {Environment} from "testcontainers/build/types";
import { randomBytes } from "crypto";
import Wallet from 'ethereumjs-wallet'
import {generateBls12381G2KeyPair} from "@mattrglobal/bbs-signatures";



export async function getNewKeys() {
    const key = randomBytes(32).toString("hex");
    const keyp =  await generateBls12381G2KeyPair();
    const wbls = Wallet.fromPrivateKey(Buffer.from(keyp.secretKey));
    const wecdsa = Wallet.fromPrivateKey(Buffer.from(key, 'hex'));
    return  { edcsa : await  wecdsa.toV3(""), bls:  await wbls.toV3("") };
}

export class DockerUtils{
    container?: StartedTestContainer;
    containerName: string;
    FINALIZER_IMAGE: string;
    constructor() {
        this.container = undefined;
        this.containerName = "";
        this.FINALIZER_IMAGE = "mangatasolutions/avs-finalizer:" + process.env.AVS_FINALIZER_VERSION || 'local';
        console.info("Using image: " + this.FINALIZER_IMAGE);
    }
    async startContainer(image: string = this.FINALIZER_IMAGE, env = this.finalizerLocalEnvironment) {
        this.containerName = image;
        const json = await getNewKeys();
        console.info("keys: " + JSON.stringify(json));
        env.ECDSA_KEY_JSON =  JSON.stringify(json.edcsa);
        env.BLS_KEY_JSON =  JSON.stringify(json.bls);
        console.info("Starting container: " + image);
        if(!this.container){
            this.container = await new GenericContainer(image)
                .withWaitStrategy(Wait.forLogMessage("Testnet setup sucessfully, starting AVS verification"))
                .withEnvironment(env)
                .withNetworkMode("host")
                .withName("rollup-avs-finalizer-TEST-" + randomBytes(4).toString("hex"))
                .withLogConsumer(stream => {
                    stream.on("data", line => console.debug(line));
                    stream.on("err", line => console.debug(line));
                    stream.on("end", () => console.debug("Stream closed"));
                })
                .start();
        }else{
            console.info("Container already started: " + this.container.getName());
        }
        return this.container;
    }
    async stopContainer() {
        console.info("Stopping container .... " + this.containerName);
        await this.container!.stop();
        console.info(".... Done! ");
    }

    finalizerLocalEnvironment : Environment = {
        RUST_LOG: "info",
        ETH_RPC_URL:"http://0.0.0.0:8545" ,
        ETH_WS_URL:"ws://0.0.0.0:8545" ,
        CHAIN_ID:"31337" ,
        SUBSTRATE_RPC_URL:"ws://0.0.0.0:9946" ,
        AVS_RPC_URL:"http://0.0.0.0:8090" ,
        AVS_REGISTRY_COORDINATOR_ADDR:"0x851356ae760d987E095750cCeb3bC6014560891C" ,
        TESTNET:"true",
        STAKE:"32",
    }
    corruptedFinalizerLocalEnvironment : Environment = {
        RUST_LOG: "info",
        ETH_RPC_URL:"http://0.0.0.0:8545" ,
        ETH_WS_URL:"ws://0.0.0.0:8545" ,
        CHAIN_ID:"31337" ,
        SUBSTRATE_RPC_URL:"wss://kusama-archive.mangata.online:443" ,
        AVS_RPC_URL:"http://0.0.0.0:8090" ,
        AVS_REGISTRY_COORDINATOR_ADDR:"0x851356ae760d987E095750cCeb3bC6014560891C" ,
        TESTNET:"true",
        STAKE:"90",
    }
}