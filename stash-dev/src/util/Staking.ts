import BigNumber from 'bignumber.js'
import moment from 'moment'

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
