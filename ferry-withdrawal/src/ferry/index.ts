import { L1Interface } from "../l1";
import { L2Interface } from "../l2";
import { Deposit } from "../common/deposit.js";
import { u8aToHex } from "@polkadot/util";

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
	minProfit: bigint;

	constructor(
		me: Uint8Array,
		l1: L1Interface,
		l2: L2Interface,
		txCost: bigint,
		minProfit: bigint,
	) {
		this.me = me;
		this.l1 = l1;
		this.l2 = l2;
		this.txCost = txCost;
		this.minProfit = minProfit;
	}

	// async hasFundsToCoverTxFee() {
	// 	const balances = await this.l2.getBalances(this.me);
	// 	const tokenAddressToBalance = new Map<string, bigint>(
	// 		Array.from(balances, ([k, v]) => [u8aToHex(k), v]),
	// 	);
	//
	// 	const nativeToken = u8aToHex(await this.l2.getNativeTokenAddress());
	// 	if (!tokenAddressToBalance.has(nativeToken)) {
	// 		return false;
	// 	} else {
	// 		return tokenAddressToBalance.get(nativeToken)! >= this.txCost;
	// 	}
	// }
	//
	logFilteredOut(before: Deposit[], after: Deposit[], message: string) {
		const diff = before.length - after.length;
		if (diff > 0) {
			console.info(`filtered out ${diff} : ${message}`);
		}
	}

	// async getPendingDeposits(): Promise<Deposit[]> {
	// 	const end = await this.l1.getLatestRequestId();
	// 	let start = await this.l2.getLastProcessedRequestId();
	// 	if (end === null) {
	// 		return Promise.resolve([]);
	// 	}
	// 	if (start === 0n) {
	// 		start = 1n;
	// 	}
	// 	const deposits = await this.l1.getDeposits(start, end);
	//
	// 	const ferryableDeposits = await asyncFilter(
	// 		deposits,
	// 		async (elem: Deposit) => {
	// 			const hash = await this.l1.getDepostiHash(elem.requestId);
	// 			const isFerried = await this.l2.isFerried(hash);
	// 			const isExecuted = await this.l2.isExecuted(elem.requestId);
	// 			return !isFerried && !isExecuted;
	// 		},
	// 	);
	// 	return ferryableDeposits;
	// }
	
	// async rateDeposits(deposits: Deposit[]): Promise<Deposit[]> {
	// 	const nativeTokenAddress = await this.l2.getNativeTokenAddress();
	// 	const balances = await this.l2.getBalances(this.me);
	//
	// 	const tokenAddressToBalance = new Map<string, bigint>(
	// 		Array.from(balances, ([k, v]) => [u8aToHex(k), v]),
	// 	);
	//
	// 	const validDeposits = deposits.filter((deposit) => {
	// 		return deposit.amount >= deposit.ferryTip;
	// 	});
	// 	this.logFilteredOut(deposits, validDeposits, "invalid deposit");
	//
	// 	const affordableDeposits = validDeposits.filter((deposit) => {
	// 		const transferAmount = deposit.amount - deposit.ferryTip;
	// 		const tokenAddress = u8aToHex(deposit.tokenAddress);
	//
	// 		if (tokenAddressToBalance.has(tokenAddress)) {
	// 			if (deposit.tokenAddress === nativeTokenAddress) {
	// 				return (
	// 					tokenAddressToBalance.get(tokenAddress)! >=
	// 					transferAmount + this.txCost
	// 				);
	// 			} else {
	// 				return tokenAddressToBalance.get(tokenAddress)! >= transferAmount;
	// 			}
	// 		} else {
	// 			return false;
	// 		}
	// 	});
	//
	// 	this.logFilteredOut(
	// 		validDeposits,
	// 		affordableDeposits,
	// 		`not enought tokens to cover Account: ${u8aToHex(this.me)}`,
	// 	);
	//
	// 	const ratedDeposits = affordableDeposits.sort((a, b) => {
	// 		if (a.ferryTip > b.ferryTip) {
	// 			return -1;
	// 		} else if (a.ferryTip === b.ferryTip) {
	// 			if (a.amount > b.amount) {
	// 				return 1;
	// 			} else if (a.amount > b.amount) {
	// 				return 0;
	// 			} else {
	// 				return -1;
	// 			}
	// 		} else {
	// 			return 1;
	// 		}
	// 	});
	//
	// 	const aboveProfitThreshold = await asyncFilter(
	// 		ratedDeposits,
	// 		async (elem: Deposit) => {
	// 			return (
	// 				(await this.l2.valutateToken(elem.tokenAddress, elem.ferryTip)) >=
	// 				this.minProfit
	// 			);
	// 		},
	// 	);
	//
	// 	this.logFilteredOut(
	// 		affordableDeposits,
	// 		aboveProfitThreshold,
	// 		"profit below expected threshold",
	// 	);
	//
	// 	return aboveProfitThreshold;
	// }
}

export { Ferry };
