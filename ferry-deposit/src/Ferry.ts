import { L1Interface } from "./l1/L1Interface.js";
import { L2Interface } from "./l2/L2Interface.js";
import { Deposit, toString } from "./Deposit.js";
import { isEqual } from "./utils.js";
import { u8aToHex } from "@polkadot/util";
import { logger } from "./logger.js";

async function asyncFilter(arr: Deposit[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => {
		return results[index];
	});
}

class Ferry {
	l1: L1Interface;
	l2: L2Interface;
	me: Uint8Array;
	txCost: bigint;
	minProfit: [Uint8Array, bigint, bigint][];

	constructor(
		me: Uint8Array,
		l1: L1Interface,
		l2: L2Interface,
		txCost: bigint,
		minProfit: [Uint8Array, bigint, bigint][],
	) {
		this.me = me;
		this.l1 = l1;
		this.l2 = l2;
		this.txCost = txCost;
		this.minProfit = minProfit;
	}

	async hasFundsToCoverTxFee() {
		const balances = await this.l2.getBalances(this.me);
		const tokenAddressToBalance = new Map<string, bigint>(
			Array.from(balances, ([k, v]) => [u8aToHex(k), v]),
		);

		const nativeToken = u8aToHex(await this.l2.getNativeTokenAddress());
		if (!tokenAddressToBalance.has(nativeToken)) {
			return false;
		} else {
			return tokenAddressToBalance.get(nativeToken)! >= this.txCost;
		}
	}

	logFilteredOut(before: Deposit[], after: Deposit[], message: string) {
		const diff = before.length - after.length;
		const mapAfter: Map<bigint, Deposit> = new Map(
			after.map((elem) => [elem.requestId, elem]),
		);

		if (diff > 0) {
			before
				.filter((elem) => mapAfter.has(elem.requestId) === false)
				.forEach((elem) =>
					logger.silly(`filtered out ${message}: ${toString(elem)}`),
				);

			logger.debug(`filtered out ${diff} withdrawals, reason "${message}"`);
		}
	}

	async getPendingDeposits(): Promise<Deposit[]> {
		const end = await this.l1.getLatestRequestId();
		let start = await this.l2.getLastProcessedRequestId();
		if (end === null) {
			return Promise.resolve([]);
		}
		if (start === 0n) {
			start = 1n;
		}
		const deposits = await this.l1.getDeposits(start, end);

		const ferryableDeposits = await asyncFilter(
			deposits,
			async (elem: Deposit) => {
				const hash = await this.l1.getDepostiHash(elem.requestId);
				const isFerried = await this.l2.isFerried(hash);
				const isExecuted = await this.l2.isExecuted(elem.requestId);
				return !isFerried && !isExecuted;
			},
		);
		return ferryableDeposits;
	}

	async rateDeposits(deposits: Deposit[]): Promise<Deposit[]> {
		const nativeTokenAddress = await this.l2.getNativeTokenAddress();
		const balances = await this.l2.getBalances(this.me);

		balances.forEach(([tokenAddress, balance]) => {
			logger.silly(`\tBalance ${u8aToHex(tokenAddress)} : ${balance}`);
		});

		const validDeposits = deposits.filter((deposit) => {
			return deposit.amount >= deposit.ferryTip;
		});
		this.logFilteredOut(deposits, validDeposits, "invalid deposit");

		const whitelisted = validDeposits.filter((deposit) =>
			this.minProfit.find(([token, minProfit, _weight]) => {
				return (
					isEqual(token, deposit.tokenAddress) !== undefined &&
					deposit.ferryTip >= minProfit
				);
			}),
		);

		this.logFilteredOut(
			validDeposits,
			whitelisted,
			"below min expected profit",
		);

		const affordableDeposits = whitelisted.filter(
			(deposit) =>
				balances.find(([token, balance]) => {
					if (isEqual(deposit.tokenAddress, token)) {
						if (isEqual(token, nativeTokenAddress)) {
							return balance >= this.txCost + deposit.amount - deposit.ferryTip;
						} else {
							return balance >= deposit.amount - deposit.ferryTip;
						}
					}
					return false;
				}) !== undefined,
		);

		this.logFilteredOut(
			validDeposits,
			affordableDeposits,
			`not enought tokens to cover Account: ${u8aToHex(this.me)}`,
		);

		const ratedDeposits = affordableDeposits.sort((a, b) => {
			const aWeight =
				this.minProfit.find(
					([token, _minProfit, _weight]) =>
						isEqual(token, a.tokenAddress) !== undefined,
				)?.[2] ?? 0n;
			const bWeight =
				this.minProfit.find(
					([token, _minProfit, _weight]) =>
						isEqual(token, b.tokenAddress) !== undefined,
				)?.[2] ?? 0n;
			const aProfit = a.ferryTip * aWeight;
			const bProfit = b.ferryTip * bWeight;
			if (aProfit > bProfit) {
				return -1;
			} else if (aProfit === bProfit) {
				if (a.amount > b.amount) {
					return 1;
				} else if (a.amount === b.amount) {
					return 0;
				} else {
					return -1;
				}
			} else {
				return 1;
			}
		});

		return ratedDeposits;
	}
}

export { Ferry };
