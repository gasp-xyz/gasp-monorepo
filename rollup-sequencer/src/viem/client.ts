import { type PublicClientConfig, createPublicClient } from "viem";

export const getPublicClient = (options: PublicClientConfig) => {
	return createPublicClient({ ...options });
};
