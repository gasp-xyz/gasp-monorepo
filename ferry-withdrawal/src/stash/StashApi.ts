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

const responseSchema = z.object({
	createdBy: z.enum([Origin.Frontend, Origin.Other]),
});

export class StashApi implements StashInterface {
	uri: string;

	constructor(uri: string) {
		this.uri = uri;
	}

	async shouldBeClosed(txHash: Uint8Array): Promise<boolean> {
		try {
			const uri = `${this.uri}/tracing/createdBy/type/withdrawal/tx/${u8aToHex(
				txHash,
			)}`;
			logger.silly(`Making stash querry : ${uri}`);
			const rawResponse = await axios.get(uri, { timeout: 5000 });
			const response = responseSchema.parse(rawResponse.data); // This will throw an error if validation fails
			logger.silly(`Withdrawal origin: ${response.createdBy}`);
			return Promise.resolve(response.createdBy === Origin.Frontend);
		} catch (error) {
			if (axios.isAxiosError(error)) {
				logger.error("Axios Error:", error.message);
				if (error.response) {
					logger.error("Response Data:", error.response.data);
					logger.error("Response Status:", error.response.status);
				} else if (error.request) {
					logger.error(
						"Request was made but no response received:",
						error.request,
					);
				} else {
					logger.error("Unexpected error:", error.message);
				}
			} else if (error instanceof ZodError) {
				console.error("Validation Failed:", error.errors);
			} else {
				logger.error("Unexpected Error:", error);
			}
			return Promise.resolve(false);
		}
	}
}
