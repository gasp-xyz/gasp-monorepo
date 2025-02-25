import axios from "axios";
import type { StashInterface } from "./StashInterface.js";
import { z, ZodError } from "zod";
import { logger } from "./../logger.js";
import { u8aToHex } from "@polkadot/util";

// GET /tracing/createdBy/type/{type}/tx/{txHash} - for you
// response:
// createdBy: frontend/other (other is default)

enum Origin {
	Frontend = "frontend",
	Other = "other",
}

export const stashResponseSchema = z.object({
	transaction: z.object({
		createdBy: z.enum([Origin.Frontend, Origin.Other]),
	}),
});

export class StashApi implements StashInterface {
	uri: string;

	constructor(uri: string) {
		this.uri = uri;
	}

	async shouldBeClosed(txHash: Uint8Array): Promise<boolean> {
		try {
			const uri = `${this.uri}/tracing/type/withdrawal/tx/${u8aToHex(txHash)}`;
			logger.silly(`Making stash querry : ${uri}`);
			const rawResponse = await axios.get(uri, { timeout: 15000 });
			const response = stashResponseSchema.parse(rawResponse.data); // This will throw an error if validation fails
			logger.silly(`Withdrawal origin: ${response.transaction.createdBy}`);
			return Promise.resolve(
				response.transaction.createdBy === Origin.Frontend,
			);
		} catch (error) {
			if (axios.isAxiosError(error)) {
				logger.error("Axios Error:", error.toJSON());
			} else if (error instanceof ZodError) {
				logger.warn("ALERT::WARN Stash response validation failed: ", error);
			} else {
				logger.error("Unexpected Error:", error);
			}
			return Promise.resolve(false);
		}
	}
}
