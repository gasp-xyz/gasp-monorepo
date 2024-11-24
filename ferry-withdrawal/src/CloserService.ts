
import { u8aToHex } from "@polkadot/util";

import { L1Interface } from "./l1/L1Interface.js";
import { L2Interface } from "./l2/L2Interface.js";
import { isEqual, maxBigInt, minBigInt} from "./utils.js";
import { logger } from "./logger.js";
import { Withdrawal, toString } from "./Withdrawal.js";
import { Cancel } from "./Cancel.js";

async function asyncFilter(arr: (Withdrawal| Cancel)[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => {
		return results[index];
	});
}

class CloserService {
	l1: L1Interface;
	l2: L2Interface;
	tokensToClose: [Uint8Array, bigint, bigint][];
  minBalance: bigint;
  lastCheckedWithrdawal : bigint;
  withdrawalsToClose : Withdrawal[];
  batchSize: bigint;

	constructor(
		l1: L1Interface,
		l2: L2Interface,
		tokensToClose: [Uint8Array, bigint, bigint][],
    minBalance: bigint,
    batchSize: bigint = 1000n,
	) {
		this.l1 = l1;
		this.l2 = l2;
		this.tokensToClose = tokensToClose;
    this.minBalance = minBalance;
    this.lastCheckedWithrdawal = 0n;
    this.withdrawalsToClose = [];
    this.batchSize = batchSize;
	}


  async findWithdrawalsToClose() : Promise<void> {
    const latestClosableReqeustIdOnL1 = await this.l1.getLatestRequestId();

    if (latestClosableReqeustIdOnL1 === null) {
      logger.debug(`No withdrawals have been brought yet to L1 contract`);
      return;
    }

    while (this.withdrawalsToClose.length === 0 && this.lastCheckedWithrdawal < latestClosableReqeustIdOnL1) {
      const rangeStart: bigint = maxBigInt(1n, this.lastCheckedWithrdawal);
      const rangeEnd: bigint = minBigInt(rangeStart + this.batchSize, latestClosableReqeustIdOnL1);

      const withdrawals = await this.l2.getRequests(rangeStart, rangeEnd);
      logger.silly(`Fetching withdrawals in range ${rangeStart} .. ${rangeEnd} - found ${withdrawals.length} withdrawals`);

      const closableWithdrawals = await asyncFilter(withdrawals, async (withdrawal: Withdrawal|Cancel) => {
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
      const {range, root} = await this.l1.getMerkleRange(withdrawal.requestId);
      const proof = await this.l2.getMerkleProof(range[0], range[1], withdrawal.requestId);
      await this.l1.close(withdrawal, root, proof, privateKey);
    }
	}

}

export { CloserService };
