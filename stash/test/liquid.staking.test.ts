import {
  rewards24hours,
  rewardsMonth,
} from '../src/service/LiquidityStakingService'
import { describe, it, vi, expect, beforeAll, afterAll } from 'vitest'
import {
  ProofOfStakeReward,
  ResponseRewards,
} from '../src/repository/StakingRepository'
import moment from 'moment'

describe('[Staking]', () => {
  beforeAll(() => {
    vi.mock('../src/repository/StakingRepository', async () => {
      const actual = await vi.importActual(
        '../src/repository/StakingRepository'
      )
      return {
        ...actual,
        getDataByAddress: vi.fn().mockImplementation((address: string) => {
          return new Promise((resolve) => {
            const d = moment.utc()
            const proofOfStakeRewards: ProofOfStakeReward[] = [
              {
                timestamp: d.valueOf().toString(),
                data: {
                  timestamp: d.valueOf(),
                  block: 2451944,
                  section: 'proofOfStake',
                  method: 'RewardsClaimed',
                  account: address,
                  liquidityTokenId: '8',
                  amountClaimed: '503619012693557062128',
                },
              },
              {
                timestamp: d.subtract(4, 'days').valueOf().toString(),
                data: {
                  timestamp: 1697148000000,
                  block: 2451944,
                  section: 'proofOfStake',
                  method: 'RewardsClaimed',
                  account: address,
                  liquidityTokenId: '8',
                  amountClaimed: '200619012693557062128',
                },
              },
              {
                timestamp: '1701259798965',
                data: {
                  timestamp: 1701259798965,
                  block: 2451954,
                  section: 'proofOfStake',
                  method: 'RewardsClaimed',
                  account: address,
                  liquidityTokenId: '8',
                  amountClaimed: '519190049058532409357',
                },
              },
              {
                timestamp: d.subtract(35, 'days').valueOf().toString(),
                data: {
                  timestamp: d.subtract(35, 'days').valueOf(),
                  block: 2451960,
                  section: 'proofOfStake',
                  method: 'RewardsClaimed',
                  account: address,
                  liquidityTokenId: '17',
                  amountClaimed: '2000163977501840511647',
                },
              },
              {
                timestamp: d.subtract(35, 'days').valueOf().toString(),
                data: {
                  timestamp: d.subtract(35, 'days').valueOf(),
                  block: 2451964,
                  section: 'proofOfStake',
                  method: 'RewardsClaimed',
                  account: address,
                  liquidityTokenId: '27',
                  amountClaimed: '520392255103853386076',
                },
              },
            ]
            resolve(proofOfStakeRewards || [])
          })
        }),
      }
    })
  })

  afterAll(async () => {
    vi.unmock('../src/repository/StakingRepository')
  })

  it('should mock the 24 hours rewards history endpoint', async () => {
    const expectedResponse: ResponseRewards[] = [
      { liquidityTokenId: '8', amountClaimed: '503619012693557062128' },
    ]
    const results = await rewards24hours(
      '5FFTemNzVqVduFk7n8z7G6qukrnBfTTQRE8EGPbjmtdpz2c1'
    )
    expect(results).deep.equal(expectedResponse)
  })

  it('should mock the month rewards history endpoint', async () => {
    const expectedResponse: ResponseRewards[] = [
      { liquidityTokenId: '8', amountClaimed: '704238025387114124256' },
    ]
    const results = await rewardsMonth(
      '5FFTemNzVqVduFk7n8z7G6qukrnBfTTQRE8EGPbjmtdpz2c1'
    )
    expect(results).deep.equal(expectedResponse)
  })
})
