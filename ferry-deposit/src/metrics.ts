import express, { Express, Request, Response } from "express";
import promClient, { Gauge } from "prom-client";
import { L2Interface } from "./l2/L2Interface";
import { sleep } from "./utils.js";

const register = new promClient.Registry();
const DELAY_5M = 300 * 1000;

export const account_balance_metric = new Gauge({
	name: "Account balance",
	help: "Account native balance",
	labelNames: ["balance"],
});

export async function serveMetrics() {
	const app: Express = express();
	app.get("/", async (req: Request, res: Response) => {
		res.setHeader("Content-Type", register.contentType);
		res.send(await register.metrics());
	});
	app.listen(80);
}

export async function reportBalance(l2: L2Interface) {
	while (true) {
		let getNativeTokenAddress = await l2.getNativeTokenAddress();
		let [_, nativeBalance] = await l2.getBalance(getNativeTokenAddress);
		account_balance_metric.set(Number(nativeBalance));
		await sleep(DELAY_5M);
	}
}
