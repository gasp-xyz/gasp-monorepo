import { createConfig } from "./config.js";

function getConfig() {
	// Here we can distinguish between different
	// configs like - local, test, staging, production
	return createConfig();
}

export const configuration = getConfig();
