import BigNumber from 'bignumber.js'
import moment from 'moment'
import { Session } from '../repository/StakingRepository'

const BG_HUNDRED = new BigNumber('100')

export function calculateAnnualPercentageYieldPerSession(
  rewards: BigNumber,
  stake: BigNumber
) {
  return rewards.dividedBy(stake).multipliedBy(BG_HUNDRED)
}

export function calculateAnnualPercentageYield(
  totalStake: BigNumber,
  totalRewards: BigNumber
) {
  return totalRewards
    .dividedBy(totalStake)
    .multipliedBy(new BigNumber(365))
    .multipliedBy(new BigNumber(100))
}

export function convertDateToTimestamp(date: string | undefined) {
  if (!date) return null
  const parsedDate = moment(date, 'DD-MM-YYYY')

  return parsedDate.valueOf()
}

export const groupDataForCollatorsApy = (data: Session[]) => {
  const grouped = {}

  data.forEach((entry) => {
    const collator = entry.data.collatorAccount
    const tokenId = entry.data.liquidityTokenId

    if (!grouped[collator]) {
      grouped[collator] = {}
    }

    if (!grouped[collator][tokenId]) {
      grouped[collator][tokenId] = []
    }

    grouped[collator][tokenId].push(entry)
  })

  return grouped
}
