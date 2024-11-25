
import { u8aToHex } from "@polkadot/util";

import { L1Interface } from "./l1/L1Interface.js";
import { L2Interface } from "./l2/L2Interface.js";
import { isEqual, maxBigInt, minBigInt} from "./utils.js";
import { logger } from "./logger.js";
import { Withdrawal, toString, isWithdrawal } from "./Withdrawal.js";
import { Cancel, isCancel } from "./Cancel.js";
import { closeSync } from "fs";

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
  closableRequests : (Withdrawal|Cancel)[];
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
    this.closableRequests = [];
    this.batchSize = batchSize;
	}


  async findRequestToClose(delay: bigint = 0n) : Promise<void> {
    const delayedLatestClosableReqeustIdOnL1 = await this.l1.getLatestRequestId(delay);

    const latestClosableReqeustIdOnL1 = await this.l1.getLatestRequestId(0n);

    if (latestClosableReqeustIdOnL1 === null) {
      logger.debug(`No withdrawals have been brought yet to L1 contract`);
      return;
    }

    while (this.closableRequests.length === 0 && this.lastCheckedWithrdawal < latestClosableReqeustIdOnL1) {
      const rangeStart: bigint = maxBigInt(1n, this.lastCheckedWithrdawal);
      const rangeEnd: bigint = minBigInt(rangeStart + this.batchSize, latestClosableReqeustIdOnL1);

      const requests = await this.l2.getRequests(rangeStart, rangeEnd);
      logger.silly(`Fetching withdrawals in range ${rangeStart} .. ${rangeEnd} - found ${requests.length} requests`);

      const closableRequests = await asyncFilter(requests, async (request: Withdrawal|Cancel) => {

        if (isCancel(request)) {
          return !(await this.l1.isClosed(request.hash));
        } else if (isWithdrawal(request)) {
          const shouldBeClosed = this.tokensToClose.find( (elem) => {
            return isEqual(elem[0], request.tokenAddress) && request.ferryTip >= elem[1];
          }) !== undefined;
          return shouldBeClosed && !(await this.l1.isClosed(request.hash));
        } else {
          logger.error(`ignoring unkonwn request`);
          return false;
        }

      });

      logger.debug(`There are ${closableRequests.length} ready to be closed`);

      const requestsToClose =  closableRequests.filter( (request) => {
        return delayedLatestClosableReqeustIdOnL1 ? request.requestId <= delayedLatestClosableReqeustIdOnL1 : false;
      });

      let ignoredCount = closableRequests.length - requestsToClose.length;
      logger.debug(`There are ${requestsToClose.length} that are old enough to be closed (and ${ignoredCount} ignored because of block delay == ${delay})`);

      this.lastCheckedWithrdawal = rangeEnd;
      this.closableRequests = requestsToClose;
    }
  }

  async getNextRequestToClose() : Promise<Cancel | Withdrawal | null> {
    return this.closableRequests.shift() ?? null;
  }

	async closeRequest(request: Withdrawal | Cancel, privateKey: Uint8Array): Promise<void> {

    if (isWithdrawal(request)) {
      await this.closeWithdrawal(request, privateKey);
    }else if (isCancel(request)) {
      await this.closeCancel(request, privateKey);
    }
	}


	async closeWithdrawal(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<void> {
    const isClosed = await this.l1.isClosed(withdrawal.hash);
    const isFerried = await this.l1.isFerried(withdrawal.hash) 
    if ( !isClosed && !isFerried) {
      const {range, root} = await this.l1.getMerkleRange(withdrawal.requestId);
      const proof = await this.l2.getMerkleProof(range[0], range[1], withdrawal.requestId);
      await this.l1.closeWithdrawal(withdrawal, root, proof, privateKey);
    }
	}


	async closeCancel(cancel: Cancel, privateKey: Uint8Array): Promise<void> {
    const isClosed = await this.l1.isClosed(cancel.hash);
    const isFerried = await this.l1.isFerried(cancel.hash) 
    if ( !isClosed && !isFerried) {
      const {range, root} = await this.l1.getMerkleRange(cancel.requestId);
      const proof = await this.l2.getMerkleProof(range[0], range[1], cancel.requestId);
      await this.l1.closeCancel(cancel, root, proof, privateKey);
    }
	}

}

export { CloserService };
