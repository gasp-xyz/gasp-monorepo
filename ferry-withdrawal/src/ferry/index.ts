import { L1Interface } from "../l1";
import { L2Interface } from "../l2";
import { Deposit } from "../common/deposit.js";
import util from "node:util";
import { u8aToHex } from "@polkadot/util";
import { Withdrawal } from "../common/withdrawal";

async function asyncFilter(arr: Withdrawal[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => {
		return results[index];
	});
}

class Ferry {
	l1: L1Interface;
	l2: L2Interface;
	me: Uint8Array;
	tokensToFerry: [Uint8Array, bigint, bigint][];
  minBalance: bigint;

	constructor(
		me: Uint8Array,
		l1: L1Interface,
		l2: L2Interface,
		tokensToFerry: [Uint8Array, bigint, bigint][],
    minBalance: bigint
	) {
		this.me = me;
		this.l1 = l1;
		this.l2 = l2;
		this.tokensToFerry = tokensToFerry;
    this.minBalance = minBalance;
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

	async getPendingWithdrawals(): Promise<Withdrawal[]> {
		const end = await this.l2.getLatestRequestId();
		const start = await this.l1.getLatestRequestId();

		if (start === null || end === null) {
			return Promise.resolve([]);
		}

		const withdrawals = await this.l2.getWithdrawals(start, end);
    const validWithdrawal = withdrawals.filter((elem) => 
      elem.requestId >= start && elem.requestId <= end && elem.amount >= elem.ferryTip
    );

		const ferryableDeposits = await asyncFilter(
			validWithdrawal,
			async (elem: Withdrawal) => {
				const isFerried = await this.l1.isFerried(elem.hash);
				const isClosed = await this.l1.isClosed(elem.hash);
				return !isFerried && !isClosed;
			},
		);
		return ferryableDeposits;
	}
	
	async rateWithdrawals(withdrawals: Withdrawal[]): Promise<Withdrawal[]> {
    const balances = await Promise.all(this.tokensToFerry.map( ([tokenAddress, _minProfit, _weight]) => Promise.all([Promise.resolve( tokenAddress), this.l1.getBalance(tokenAddress, this.me)])));
    const nativeTokenAddress = await this.l2.getNativeTokenAddress();

		const canAffordWirhdrawals = withdrawals.filter((elem) => 
			balances.find(([tokenAddress, balance]) => 
      {
          if (tokenAddress === nativeTokenAddress) {
            return tokenAddress === elem.tokenAddress && balance != null && balance >= (elem.amount + this.minBalance);
          }else{
            return tokenAddress === elem.tokenAddress && balance != null && balance >= elem.amount;
          }
      }) !== undefined
		);

    const rankedWithdrawals = canAffordWirhdrawals.sort((first, second) => {
      const firstTokenWeight = this.tokensToFerry.find(([tokenAddress, minProfit, weight]) => tokenAddress === first.tokenAddress)![2];
      const secondTokenWeight = this.tokensToFerry.find(([tokenAddress, minProfit, weight]) => tokenAddress === second.tokenAddress)![2];
      const firstWithdrawalProfit = first.ferryTip * firstTokenWeight;
      const secondWithdrawalProfit = second.ferryTip * secondTokenWeight;

      if (firstWithdrawalProfit > secondWithdrawalProfit) {
        return -1;
      }  else if (firstWithdrawalProfit < secondWithdrawalProfit) {
        return 1;
      }else { // equal profits
        if (first.amount > second.amount) {
          return -1;
        }else if (first.amount < second.amount) {
          return 1;
        } else {
          return 0;
        }
      }
    });

    const result = rankedWithdrawals;
    return result;
	}
}

export { Ferry };
