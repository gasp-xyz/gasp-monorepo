import {
  get24HoursAccountRewardsData,
  getDataByAddress,
  getMonthAccountRewardsData,
  KEY_ACCOUNT,
  ProofOfStakeEntry,
  ProofOfStakeReward,
  calculateClaimedRewards,
} from '../repository/StakingRepository.js'

export const rewards24hours = async (address: string) => {
  const data: ProofOfStakeReward[] = await getDataByAddress<ProofOfStakeEntry>(
    KEY_ACCOUNT,
    address,
    (data) => data.account
  )

  const rewards24hoursData = get24HoursAccountRewardsData(data)

  return calculateClaimedRewards(rewards24hoursData)
}

export const rewardsMonth = async (address: string) => {
  const data: ProofOfStakeReward[] = await getDataByAddress<ProofOfStakeEntry>(
    KEY_ACCOUNT,
    address,
    (data) => data.account
  )

  const rewardsMonthData = getMonthAccountRewardsData(data)

  return calculateClaimedRewards(rewardsMonthData)
}
