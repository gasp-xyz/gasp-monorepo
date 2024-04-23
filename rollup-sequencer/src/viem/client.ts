import {createPublicClient, PublicClientConfig} from "viem";

export const getPublicClient = (options: PublicClientConfig) => {
    return createPublicClient({...options})
}
