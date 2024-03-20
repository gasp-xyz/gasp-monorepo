import {defineChain} from "viem";

export const anvil = defineChain({
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
            http: ["ws://0.0.0.0:8545"],
        },
        default: {
            http: ["ws://0.0.0.0:8545"],
        },
    },
});