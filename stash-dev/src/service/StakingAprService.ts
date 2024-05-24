import { NotFoundException } from '../error/Exception.js'
import {
  calculateCollatorApy,
  calculateDailyRewards,
  getDataByAddress,
  SessionData,
  ResponseAPY,
  Session,
  KEY,
} from '../repository/StakingRepository.js'

export const apy = async (collatorAddress: string): Promise<ResponseAPY> => {
  const collatorSessions: Session[] = await getDataByAddress<SessionData>(
    KEY,
    collatorAddress,
    (data) => data.collatorAccount
  )

  if (collatorSessions.length === 0)
    throw new NotFoundException(
      'This collator has not received any rewards as of yet.'
    )

  return calculateCollatorApy(collatorSessions)
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
