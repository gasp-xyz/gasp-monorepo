import {
    type PublicClientConfig,
    createPublicClient,
    type  WalletClientConfig,
    createWalletClient,
    webSocket,
    defineChain
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import {ETH_CHAIN_URL, WALLET_PRIVATE_KEY} from "../common/constants.js";
import {holesky, arbitrumSepolia} from "viem/chains";

export const getPublicClient = (options: PublicClientConfig) => {
    return createPublicClient({ ...options });
};

export const getWalletClient = (options: WalletClientConfig) => {
    return createWalletClient({ ...options });
};

export const ethAccount = privateKeyToAccount(`0x${WALLET_PRIVATE_KEY}`);
export const webSocketTransport = webSocket(ETH_CHAIN_URL, {retryCount: 5});

export function getChain() {
    if (process.env.CHAIN == "holesky") {
        return holesky
    } else if (process.env.CHAIN == "reth" || process.env.CHAIN == "reth-1337"){
        return defineChain({
            id: 1337,
            name: "reth",
            network: "Reth",
            nativeCurrency: {
                decimals: 18,
                name: "Ether",
                symbol: "ETH",
            },
            rpcUrls: {
                public: {
                    http: ["ws://127.0.0.1:8545"],
                },
                default: {
                    http: ["ws://127.0.0.1:8545"],
                },
            },
        });
    } else if (process.env.CHAIN == "reth-31337"){
        return defineChain({
            id: 31337,
            name: "reth",
            network: "Reth",
            nativeCurrency: {
                decimals: 18,
                name: "Ether",
                symbol: "ETH",
            },
            rpcUrls: {
                public: {
                    http: ["ws://127.0.0.1:8545"],
                },
                default: {
                    http: ["ws://127.0.0.1:8545"],
                },
            },
        });
    } else if (process.env.CHAIN == "arbitrum"){
        return arbitrumSepolia;
    } else {
        return defineChain({
            id: 31337,
            name: "anvil",
            network: "Anvil",
            nativeCurrency: {
                decimals: 18,
                name: "Ether",
                symbol: "ETH",
            },
            rpcUrls: {
                public: {
                    http: ["ws://127.0.0.1:8545"],
                },
                default: {
                    http: ["ws://127.0.0.1:8545"],
                },
            },
        });
    }
}
