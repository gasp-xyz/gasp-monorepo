import express, { Express, Request, Response } from "express";
import promClient, { Gauge } from "prom-client";
import { L2Interface } from "./l2/L2Interface";
import { isEqual, sleep } from "./utils.js";
import { PORT } from "./config.js";

const register = new promClient.Registry();
const DELAY_5M = 300 * 1000;

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
	app.listen(PORT);
}

export async function reportBalance(l2: L2Interface) {
	while (true) {
		let getNativeTokenAddress = await l2.getNativeTokenAddress();
		let balances = await l2.getBalance(getNativeTokenAddress);
		let maybeBalance = balances.find(([token, amount]) =>
			isEqual(token, getNativeTokenAddress),
		);
		let nativeBalance = maybeBalance ? maybeBalance[1] : 0.0;
		account_balance_metric.set(Number(nativeBalance) / 1000000000000000000.0);
		await sleep(DELAY_5M);
	}
}
