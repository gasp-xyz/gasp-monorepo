import { webSocket } from "viem";
import { ETH_CHAIN_URL } from "../common/constants.js";

export const webSocketTransport = webSocket(ETH_CHAIN_URL, { retryCount: 5 });
