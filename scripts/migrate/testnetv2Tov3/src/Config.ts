import { z } from "zod";

const cliConfigSchemat = z.object({
	SOURCE_URL: z.string(),
	TARGET_URL: z.string(),
	TARGET_SUDO_MNEMONIC: z.string(),
	LOG: z.string(),
});


function createCliConfig() {
	return cliConfigSchemat.parse({
		SOURCE_URL: process.env.SOURCE_URL!,
		TARGET_URL: process.env.TARGET_URL!,
		TARGET_SUDO_MNEMONIC: process.env.TARGET_SUDO_MNEMONIC!,
		LOG: process.env.LOG!,
	});
}

const configuration = createCliConfig();

export const SOURCE_URL = configuration.SOURCE_URL;
export const TARGET_URL = configuration.TARGET_URL;
export const TARGET_SUDO_MNEMONIC = configuration.TARGET_SUDO_MNEMONIC;
export const LOG = configuration.LOG;
