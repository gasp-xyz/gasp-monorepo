import { ApiPromise } from '@polkadot/api'
import { Codec, IEvent } from '@polkadot/types/types'
import BigNumber from 'bignumber.js'

import { redis } from '../connector/RedisConnector.js'
import {
  getCandidate,
  getCollatorAccount,
  getCollatorEvents,
  getLiquidStakingEvents,
  KEY,
  KEY_ACCOUNT,
  roundCollatorRewardInfo,
  valuateLiquidityToken,
} from '../repository/StakingRepository.js'
import { calculateAnnualPercentageYieldPerSession } from '../util/Staking.js'
import { Block } from './BlockScraper.js'

export const processStaking = async (api: ApiPromise, block: Block) => {
  // This assigns the API to a specific block hash,
  // ensuring that all other operations utilizing the 'apiAt' function
  // are executed within this particular block hash.
  const apiAt = await api.at(block.hash)

  const eventsRecord = await apiAt.query.system.events()
  const sessionIndex = await apiAt.query.parachainStaking.round()

  const collatorEvents = getCollatorEvents(eventsRecord)
  if (collatorEvents.length > 0) {
    const toCollatorEvents = collatorEvents.map(async (event) => {
      const currentSessionIndex = Number(sessionIndex.current.toString())
      const eventSessionIndex =
        event.event.data.length > 2
          ? Number(event.event.data.toPrimitive()[0])
          : 0
      const jumpInSession = currentSessionIndex - eventSessionIndex
      const numberOfBlocksToSubtract =
        jumpInSession * Number(api.consts.issuance.blocksPerRound.toString())
      const jumpToBlockForCandidate = block.number - numberOfBlocksToSubtract
      const blockHashForCandidate = await api.rpc.chain.getBlockHash(
        jumpToBlockForCandidate,
      )
      const apiAtForCandidate = await api.at(blockHashForCandidate)
      const candidate = await getCandidate(apiAtForCandidate, event)

      if (!candidate) return null

      const liquidityTokenId = candidate.liquidityToken.toString()
      const liquidityTokenAmount = new BigNumber(candidate.amount.toString())

      let valuation: {
        liquidityTokenAmountInMgx: BigNumber
        mgxTokenReserve: BigNumber
        liquidityTokenReserve: BigNumber
      }
      if (liquidityTokenId === '0') {
        const liquidityTokenReserve =
          await apiAt.query.tokens.totalIssuance(liquidityTokenId)
        valuation = {
          liquidityTokenAmountInMgx: liquidityTokenAmount,
          liquidityTokenReserve: new BigNumber(
            liquidityTokenReserve.toString(),
          ),
          mgxTokenReserve: liquidityTokenAmount,
        }
      } else {
        valuation = await valuateLiquidityToken(
          apiAt,
          liquidityTokenId,
          liquidityTokenAmount,
        )
      }
      const rewardsCollatorAmount = roundCollatorRewardInfo(event.event.data)
      const collatorAccount = getCollatorAccount(event.event.data)
      const apyPerSession = calculateAnnualPercentageYieldPerSession(
        rewardsCollatorAmount,
        valuation.liquidityTokenAmountInMgx,
      )

      return {
        timestamp: block.timestamp,
        block: block.number,
        section: event.event.section,
        method: event.event.method,
        sessionIndex: Number(sessionIndex.current.toString()),
        collatorAccount,
        amountRewarded: rewardsCollatorAmount.multipliedBy(0.8).toFixed(0),
        liquidityTokenId,
        mgxTokenReserve: valuation.mgxTokenReserve.toFixed(0),
        liquidityTokenReserve: valuation.liquidityTokenReserve.toFixed(0),
        collatorAccountAmountStakedInLp: liquidityTokenAmount.toFixed(0),
        collatorAccountAmountStakedInMgx:
          valuation.liquidityTokenAmountInMgx.toFixed(0),
        apyPerSessionInPercentage: apyPerSession,
      }
    })

    const toStore = (await Promise.all(toCollatorEvents)).filter(
      (event) => event !== null,
    )
    const storeInRedis = toStore
      .map((e) => [e.timestamp, JSON.stringify(e)])
      .flat()

    if (storeInRedis.length > 0) {
      await redis.client.zadd(KEY, ...storeInRedis)
    }
  }
}

export const processLiquidStaking = async (api: ApiPromise, block: Block) => {
  const apiAt = await api.at(block.hash)
  const eventsRecord = await apiAt.query.system.events()
  const liquidStakingEvents = getLiquidStakingEvents(eventsRecord)
  if (liquidStakingEvents.length > 0) {
    const toLiquidStakingEvents = liquidStakingEvents.map(async (event) => {
      const data: IEvent<Codec[]>['data'] = event.event.data
      const account = data[0].toString()
      const liquidityTokenId = data[1].toString()
      const amountClaimed = data[2].toString()

      return {
        timestamp: block.timestamp,
        block: block.number,
        section: event.event.section,
        method: event.event.method,
        account,
        liquidityTokenId,
        amountClaimed,
      }
    })

    const toStore = await Promise.all(toLiquidStakingEvents)
    const storeInRedis = toStore
      .map((e) => [e.timestamp, JSON.stringify(e)])
      .flat()

    if (storeInRedis.length > 0) {
      await redis.client.zadd(KEY_ACCOUNT, ...storeInRedis)
    }
  }
}
