
import { u8aToHex } from "@polkadot/util";

import { L1Interface } from "./l1/L1Interface.js";
import { L2Interface } from "./l2/L2Interface.js";
import { isEqual, minBigInt} from "./utils.js";
import { logger } from "./logger.js";
import { Withdrawal, toString } from "./Withdrawal.js";

async function asyncFilter(arr: Withdrawal[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => {
		return results[index];
	});
}

class CloserService {
	l1: L1Interface;
	l2: L2Interface;
	me: Uint8Array;
	tokensToClose: [Uint8Array, bigint, bigint][];
  minBalance: bigint;
  lastCheckedWithrdawal : bigint;
  withdrawalsToClose : Withdrawal[];

	constructor(
		me: Uint8Array,
		l1: L1Interface,
		l2: L2Interface,
		tokensToClose: [Uint8Array, bigint, bigint][],
    minBalance: bigint,
	) {
		this.me = me;
		this.l1 = l1;
		this.l2 = l2;
		this.tokensToClose = tokensToClose;
    this.minBalance = minBalance;
    this.lastCheckedWithrdawal = 1n;
    this.withdrawalsToClose = [];

    // this.lastCheckedWithrdawal = 0;
    // this.withdrawalsToClose = [];
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

  async findWithdrawalsToClose() : Promise<void> {
    const latestClosableReqeustIdOnL1 = await this.l1.getLatestRequestId();

    if (latestClosableReqeustIdOnL1 === null) {
      return;
    }

    while (this.withdrawalsToClose.length === 0 || this.lastCheckedWithrdawal >= latestClosableReqeustIdOnL1) {
      const rangeStart: bigint = this.lastCheckedWithrdawal;
      const rangeEnd = minBigInt(rangeStart + 100n, latestClosableReqeustIdOnL1);

      const withdrawals = await this.l2.getWithdrawals(rangeStart, rangeEnd);
      logger.silly(`Fetching withdrawals in range ${rangeStart} .. ${rangeEnd} - found ${withdrawals.length} withdrawals`);

      const closableWithdrawals = await asyncFilter(withdrawals, async (withdrawal: Withdrawal) => {
        const shouldBeClosed = this.tokensToClose.find( (elem) => {
          return isEqual(elem[0], withdrawal.tokenAddress) && withdrawal.ferryTip >= elem[1];
        }) !== undefined;
        return shouldBeClosed && !(await this.l1.isClosed(withdrawal.hash));
      });

      logger.debug(`Fetching withdrawals in range ${rangeStart} .. ${rangeEnd} - found ${closableWithdrawals.length} withdrawals`);

      this.lastCheckedWithrdawal = rangeEnd;
      this.withdrawalsToClose = closableWithdrawals;
    }
  }


  async getNextWithdrawalToClose() : Promise<Withdrawal | null> {
    return this.withdrawalsToClose.shift() ?? null;
  }

	async closeWithdrawal(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<void> {
    const isClosed = await this.l1.isClosed(withdrawal.hash);
    const isFerried = await this.l1.isFerried(withdrawal.hash) 
    if ( !isClosed && !isFerried) {
      await this.l1.close(withdrawal, privateKey);
    }
	}

}

export { CloserService };
