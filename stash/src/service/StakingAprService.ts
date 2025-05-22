import { NotFoundException } from '../error/Exception.js'
import {
  calculateDailyRewards,
  calculateMultipleCollatorApy,
  calculateSingleCollatorApy,
  getDataByAddress,
  getTimeSeriesRedisData,
  KEY,
  ResponseAPY,
  Session,
  SessionData,
} from '../repository/StakingRepository.js'
import { groupDataForCollatorsApy } from '../util/Staking.js'

export const apy = async (collatorAddress: string): Promise<ResponseAPY[]> => {
  const collatorSessions: Session[] = await getDataByAddress<SessionData>(
    KEY,
    collatorAddress,
    (data) => data.collatorAccount
  )

  if (collatorSessions.length === 0)
    throw new NotFoundException(
      'This collator has not received any rewards as of yet.'
    )

  return calculateMultipleCollatorApy(collatorSessions)
}

export const collatorsApy = async (): Promise<Session[]> => {
  const data = await getTimeSeriesRedisData(KEY)
  const result: { timestamp: string; data: SessionData }[] = []

  for (let i = 0; i < data.length; i += 2) {
    const score = data[i + 1]
    const parsedData: SessionData = JSON.parse(data[i])
    result.push({ timestamp: score, data: parsedData })
  }

  const collatorsApy = []
  const groupSessions = groupDataForCollatorsApy(result)

  for (const collator in groupSessions) {
    for (const liquidityTokenId in groupSessions[collator]) {
      const apy = calculateSingleCollatorApy(
        groupSessions[collator][liquidityTokenId]
      )
      collatorsApy.push(apy)
    }
  }

  return collatorsApy
}

export const dailyRewards = async (
  collatorAddress: string,
  date: string | undefined
) => {
  const collatorSessions: Session[] = await getDataByAddress<SessionData>(
    KEY,
    collatorAddress,
    (data) => data.collatorAccount
  )

  if (collatorSessions.length === 0)
    throw new NotFoundException(
      'This collator has not received any rewards as of yet.'
    )

  return calculateDailyRewards(collatorSessions, date)
}
