import express, { Express, Request, Response } from "express";
import promClient, { Gauge } from "prom-client";
import { L1Interface } from "./l1/L1Interface";
import { ALERT_WARNING, logger } from "./logger.js";
import { METRICS_PORT } from "./Config.js";

const register = new promClient.Registry();
const DELAY_5M = 300 * 1000;

function sleep(timeInMilliseconds: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, timeInMilliseconds));
}

export const account_balance_metric = new Gauge({
	name: "account_balance",
	help: "Account native balance",
	labelNames: ["balance"],
});
register.registerMetric(account_balance_metric);

export async function serveMetrics() {
	const app: Express = express();
	app.get("/metrics", async (req: Request, res: Response) => {
		res.setHeader("Content-Type", register.contentType);
		res.send(await register.metrics());
	});
	app.listen(METRICS_PORT);
}

export async function reportBalance(account: Uint8Array, l1: L1Interface) {
	while (true) {
		let getNativeTokenAddress = await l1.getNativeTokenAddress();
		let nativeBalance = await l1.getBalance(getNativeTokenAddress, account);
		if (nativeBalance !== null) {
			account_balance_metric.set(Number(nativeBalance) / 1000000000000000000.0);
		} else {
			logger.info(`${ALERT_WARNING} Failed to fetch account balance`);
		}
		await sleep(DELAY_5M);
	}
}
