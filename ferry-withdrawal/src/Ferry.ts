import { u8aToHex } from "@polkadot/util";

import { L1Interface } from "./l1/L1Interface.js";
import { L2Interface } from "./l2/L2Interface.js";
import { isEqual, asyncFilter} from "./utils.js";
import { logger } from "./logger.js";
import { Withdrawal, toString } from "./Withdrawal.js";


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

	logFilteredOut(before: Withdrawal[], after: Withdrawal[], message: string) {
		const diff = before.length - after.length;
    const mapAfter : Map<bigint, Withdrawal> = new Map(after.map( (elem) => [elem.requestId, elem]));


		if (diff > 0) {
      before
        .filter( elem => mapAfter.has(elem.requestId) === false)
        .forEach( elem => logger.silly(`filtered out ${message}: ${toString(elem)}`));

			logger.debug(`filtered out ${diff} withdrawals, reason "${message}"`);
		}
	}

	async getPendingWithdrawals(): Promise<Withdrawal[]> {
    const lastL1 = await this.l1.getLatestRequestId();
		const start = lastL1 ? lastL1 + 1n : 1n;
		const end = await this.l2.getLatestRequestId();

		if (end === null) {
			return Promise.resolve([]);
		}

    if (end < start) {
			return Promise.resolve([]);
    }

    logger.silly(`Looking for ferryable withdrawals in rid ${start} .. ${end}`);

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

    balances.forEach(([tokenAddress, balance]) => {
      logger.silly(`\tBalance ${u8aToHex(tokenAddress)} : ${balance}`);
    });

    const nativeTokenAddress = await this.l1.getNativeTokenAddress();


		const canAffordWirhdrawals = withdrawals.filter((elem) => 
			balances.find(([tokenAddress, balance]) => 
      {
          if (isEqual(tokenAddress , nativeTokenAddress)) {
            return isEqual(tokenAddress , elem.tokenAddress) && balance != null && balance >= (elem.amount + this.minBalance);
          }else{
            return isEqual(tokenAddress , elem.tokenAddress) && balance != null && balance >= elem.amount;
          }
      }) !== undefined
		);

    this.logFilteredOut(withdrawals, canAffordWirhdrawals, "not enough balance");
 
    const aboveMinimalProfit = canAffordWirhdrawals.filter(elem => {
      const minProfit = this.tokensToFerry.find(([tokenAddress, minProfit, weight]) => isEqual(tokenAddress ,elem.tokenAddress))![1];
      return elem.ferryTip >= minProfit;
    });

    this.logFilteredOut(canAffordWirhdrawals, aboveMinimalProfit, "below minimal profit");

    const rankedWithdrawals = aboveMinimalProfit.sort((first, second) => {
      const firstTokenWeight = this.tokensToFerry.find(([tokenAddress, minProfit, weight]) => isEqual(tokenAddress , first.tokenAddress))![2];
      const secondTokenWeight = this.tokensToFerry.find(([tokenAddress, minProfit, weight]) => isEqual(tokenAddress , second.tokenAddress))![2];
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

	async getPastFerriesReadyToClose(blockInPast: number, who: Uint8Array | null = null): Promise<Withdrawal[]> {
    const account: Uint8Array = who === null ? this.me : who!;
    const rangeEnd = await this.l1.getLatestRequestId();
    const latestRequestIdInPast = await this.l2.getLatestRequestIdInPast(blockInPast);
    const rangeStart = latestRequestIdInPast ? latestRequestIdInPast : 1n;
    if (rangeStart === null || rangeEnd === null) {
      return [];
    }

    const withdrawals = await this.l2.getWithdrawals(rangeStart!, rangeEnd!);

    const withdrawalsWithStatus: [Withdrawal, Uint8Array | null][] = await Promise.all(
      withdrawals
      .map(elem => Promise.all([Promise.resolve(elem), this.l1.getFerry(elem.hash)]))
    );

    logger.silly(`Found ${withdrawals.length} past ferried withdrawals, looked through rid (${rangeStart} ... ${rangeEnd})`);

    const result = withdrawalsWithStatus
      .filter( ([_withdrawal, ferry]) => ferry !== null && isEqual(ferry!, account))
      .map(([withdrawal, _ferry]) => withdrawal);

    logger.silly(`Found ${result.length} past ferried withdrawals ferried by ${u8aToHex(account)}`);

    return result;

	}
}

export { Ferry };
