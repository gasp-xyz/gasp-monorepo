import {
  FrameSystemEventRecord,
  ParachainStakingBond,
} from '@polkadot/types/lookup'
import { ApiDecoration } from '@polkadot/api/types'
import { Codec } from '@polkadot/types-codec/types'
import { IEvent } from '@polkadot/types/types'
import { Vec } from '@polkadot/types'
import BigNumber from 'bignumber.js'
import moment from 'moment'
import _ from 'lodash'

import { timeseries } from '../connector/RedisConnector.js'
import {
  calculateAnnualPercentageYield,
  convertDateToTimestamp,
} from '../util/Staking.js'

const EVENT_SECTION = 'parachainStaking'
const EVENT_SECTION_LIQUID_STAKING = 'proofOfStake'
const EVENT_METHOD = 'Rewarded'
const EVENT_METHOD_LIQUID_STAKING = 'RewardsClaimed'
const PREFIX = 'staking:'
export const KEY = PREFIX + 'collator_rewarded'
export const KEY_ACCOUNT = PREFIX + 'account_rewarded'

export type ResponseAPY = {
  token: string
  apy: string
  collatorAddress: string
  date: string
  dateFormat: string
  timestamp: string
}

export type DailyRewardResponse = {
  tokenId: string
  dailyRewards: string
  timestamp: number
  date: string
  dateFormat: string
}

export type ResponseRewards = {
  liquidityTokenId: string
  amountClaimed: string
}

export type SessionData = {
  timestamp: number
  block: number
  section: string
  method: string
  sessionIndex: number
  collatorAccount: string
  amountRewarded: string
  liquidityTokenId: string
  mgxTokenReserve: string
  liquidityTokenReserve: string
  collatorAccountAmountStakedInLp: string
  collatorAccountAmountStakedInMgx: string
  apyPerSessionInPercentage: string
}
export type Session = {
  timestamp: string
  data: SessionData
}

export type ProofOfStakeEntry = {
  timestamp: number
  block: number
  section: string
  method: string
  account: string
  liquidityTokenId: string
  amountClaimed: string
}

export type ProofOfStakeReward = {
  timestamp: string
  data: ProofOfStakeEntry
}

export const getTimeSeriesRedisData = async (
  key: string
): Promise<string[]> => {
  return await timeseries.client.zrangebyscore(key, '-inf', 'inf', 'WITHSCORES')
}

export const getDataByAddress = async <T>(
  key: string,
  address: string,
  getAddress: (data: T) => string
): Promise<{ timestamp: string; data: T }[]> => {
  const data = await getTimeSeriesRedisData(key)
  const result: { timestamp: string; data: T }[] = []

  for (let i = 0; i < data.length; i += 2) {
    const score = data[i + 1]
    const parsedData: T = JSON.parse(data[i])

    if (getAddress(parsedData) === address) {
      result.push({ timestamp: score, data: parsedData })
    }
  }

  return result
}

export function calculateDailyRewards(
  sessions: Session[],
  date: string | undefined
): DailyRewardResponse[] {
  const providedTimestamp = date ? convertDateToTimestamp(date) : 0

  const latestTimestamps = sessions.reduce((result, item) => {
    const { liquidityTokenId, timestamp } = item.data
    const currentTimestamp = Math.max(timestamp, providedTimestamp)

    if (
      !result[liquidityTokenId] ||
      currentTimestamp > result[liquidityTokenId]
    ) {
      result[liquidityTokenId] = currentTimestamp
    }

    return result
  }, {})

  return Object.entries(latestTimestamps).map(
    ([liquidityTokenId, latestTimestamp]) => {
      const latestSession = sessions.find(
        (item) =>
          item.data.liquidityTokenId === liquidityTokenId &&
          item.data.timestamp === latestTimestamp
      )

      const dailyRewards = new BigNumber(latestSession.data.amountRewarded)
        .multipliedBy(new BigNumber('6'))
        .toFixed(0)
      return {
        tokenId: liquidityTokenId,
        dailyRewards,
        timestamp: latestSession.data.timestamp,
        date: moment(latestSession.data.timestamp).format('DD/MM/YYYY'),
        dateFormat: 'DD/MM/YYYY',
      }
    }
  )
}

export function calculateCollatorApy(sessions: Session[]) {
  let response = {} as ResponseAPY

  for (const session of sessions) {
    const liquidityTokenId = session.data.liquidityTokenId

    if (response.token) {
      if (session.data.timestamp > parseInt(response.timestamp)) {
        response = {
          token: liquidityTokenId,
          apy: calculateAnnualPercentageYield(
            new BigNumber(session.data.collatorAccountAmountStakedInMgx),
            new BigNumber(session.data.amountRewarded).multipliedBy(
              new BigNumber('6')
            )
          ).toString(),
          collatorAddress: session.data.collatorAccount,
          date: moment(session.data.timestamp).format('DD/MM/YYYY'),
          dateFormat: 'DD/MM/YYYY',
          timestamp: session.data.timestamp.toString(),
        }
      }
    } else {
      response = {
        token: liquidityTokenId,
        apy: calculateAnnualPercentageYield(
          new BigNumber(session.data.collatorAccountAmountStakedInMgx),
          new BigNumber(session.data.amountRewarded).multipliedBy(
            new BigNumber('6')
          )
        ).toString(),
        collatorAddress: session.data.collatorAccount,
        date: moment(session.data.timestamp).format('DD/MM/YYYY'),
        dateFormat: 'DD/MM/YYYY',
        timestamp: session.data.timestamp.toString(),
      }
    }
  }

  return response
}

export function get24HoursAccountRewardsData(data: ProofOfStakeReward[]): {
  [key: string]: ProofOfStakeReward[]
} {
  const now = moment.utc().valueOf()
  const twentyFourHoursAgo = now - 24 * 60 * 60 * 1000
  return data
    .filter((entry) => parseInt(entry.timestamp) > twentyFourHoursAgo)
    .reduce((groups, entry) => {
      const liquidityTokenId = entry.data.liquidityTokenId

      if (!groups[liquidityTokenId]) {
        groups[liquidityTokenId] = []
      }

      groups[liquidityTokenId].push(entry)

      return groups
    }, {} as { [key: string]: ProofOfStakeReward[] })
}

export function getMonthAccountRewardsData(data: ProofOfStakeReward[]): {
  [key: string]: ProofOfStakeReward[]
} {
  const currentDate = moment.utc()
  const startDate = moment(currentDate).subtract(30, 'days')
  const endDate = moment(currentDate)

  return data
    .filter((entry) => {
      const entryTimestamp = parseInt(entry.timestamp)
      return (
        entryTimestamp >= startDate.valueOf() &&
        entryTimestamp <= endDate.valueOf()
      )
    })
    .reduce((groups, entry) => {
      const liquidityTokenId = entry.data.liquidityTokenId

      if (!groups[liquidityTokenId]) {
        groups[liquidityTokenId] = []
      }

      groups[liquidityTokenId].push(entry)

      return groups
    }, {} as { [key: string]: ProofOfStakeReward[] })
}

export function calculateClaimedRewards(data: {
  [key: string]: ProofOfStakeReward[]
}): ResponseRewards[] {
  return Object.entries(data).map(([liquidityTokenId, proofOfStakeRewards]) => {
    return {
      liquidityTokenId: liquidityTokenId,
      amountClaimed: proofOfStakeRewards.reduce(
        (sum, entry) =>
          new BigNumber(sum)
            .plus(new BigNumber(entry.data.amountClaimed))
            .toFixed(0),
        '0'
      ),
    }
  })
}
/**
 * We receive the amount staked by the collator in LP tokens,
 * but we need to convert this to MGX stake.
 * The calculation is performed based on the method within the Mangata node.
 * https://github.com/mangata-finance/mangata-node/blob/develop/pallets/xyk/src/lib.rs#L3537
 * */
export async function valuateLiquidityToken(
  apiAt: ApiDecoration<'promise'>,
  liquidityTokenId: string,
  liquidityTokenAmount: BigNumber
) {
  const liquidityPool = await apiAt.query.xyk.liquidityPools(liquidityTokenId)
  const [firstTokenId, secondTokenId] = liquidityPool
    .unwrap()
    .map((num) => num.toString())
  const [_, mgxTokenReserve] = await apiAt.query.xyk.pools([
    firstTokenId,
    secondTokenId,
  ])
  const liquidityTokenReserve = await apiAt.query.tokens.totalIssuance(
    liquidityTokenId
  )
  return {
    liquidityTokenAmountInMgx: new BigNumber(mgxTokenReserve.toString())
      .multipliedBy(new BigNumber(2))
      .multipliedBy(liquidityTokenAmount)
      .dividedBy(new BigNumber(liquidityTokenReserve.toString())),
    liquidityTokenReserve: new BigNumber(liquidityTokenReserve.toString()),
    mgxTokenReserve: new BigNumber(mgxTokenReserve.toString()),
  }
}
/**
 * Given the endpoint parameter,
 * we receive the collator address and must determine
 * whether this address is within the candidate pool for the current session.
 * */
export async function getCandidate(
  apiAt: ApiDecoration<'promise'>,
  event: FrameSystemEventRecord
): Promise<ParachainStakingBond> {
  // We need to retrieve the pool of collator candidates,
  // each with their respective total backing stake at the current block hash
  const candidatePool = await apiAt.query.parachainStaking.candidatePool()
  return candidatePool.find((candidate) => {
    const collatorAccount = getCollatorAccount(event.event.data)
    return candidate.owner.toPrimitive() === collatorAccount
  })
}

/**
 * Before the runtime upgrade event,
 * the event.event.data had a length of 2,
 * structured as follows: owner at index 0 and amount at index 1.
 * After the runtime upgrade,
 * the event.event.data has a length of 3,
 * organized as follows: session at index 0, owner at index 1, and amount at index 2.
 * */
export function getCollatorAccount(data: IEvent<Codec[]>['data']): string {
  const index = data.length > 2 ? 1 : 0
  return data.toPrimitive()[index]
}

export function getCollatorEvents(eventsRecord: Vec<FrameSystemEventRecord>) {
  return _.filter(
    eventsRecord,
    (event) =>
      event.event.section === EVENT_SECTION &&
      event.event.method === EVENT_METHOD
  )
}

export function getLiquidStakingEvents(
  eventsRecord: Vec<FrameSystemEventRecord>
) {
  return _.filter(
    eventsRecord,
    (event) =>
      event.event.section === EVENT_SECTION_LIQUID_STAKING &&
      event.event.method === EVENT_METHOD_LIQUID_STAKING
  )
}

/**
 * Before the runtime upgrade event,
 * the event.event.data had a length of 2,
 * structured as follows: owner at index 0 and amount at index 1.
 * After the runtime upgrade,
 * the event.event.data has a length of 3,
 * organized as follows: session at index 0, owner at index 1, and amount at index 2.
 * */
export function roundCollatorRewardInfo(data: IEvent<Codec[]>['data']) {
  const index = data.length > 2 ? 2 : 1
  return new BigNumber(data.toPrimitive()[index])
}
