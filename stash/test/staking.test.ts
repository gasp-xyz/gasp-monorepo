import { apy, dailyRewards } from '../src/service/StakingAprService'
import { describe, it, vi, expect, beforeAll, afterAll } from 'vitest'
import {
  DailyRewardResponse,
  ResponseAPY,
  Session,
} from '../src/repository/StakingRepository'

describe('[Staking]', () => {
  beforeAll(() => {
    vi.mock('../src/repository/StakingRepository', async () => {
      const actual = await vi.importActual(
        '../src/repository/StakingRepository'
      )
      return {
        ...actual,
        getDataByAddress: vi
          .fn()
          .mockImplementation((collatorAddress: string) => {
            return new Promise((resolve) => {
              const sessions: Session[] = [
                {
                  timestamp: '1692179082649',
                  data: {
                    timestamp: 1692179082649,
                    block: 2988019,
                    section: 'parachainStaking',
                    method: 'Rewarded',
                    sessionIndex: 2490,
                    collatorAccount:
                      '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
                    amountRewarded: '2987444575204336243725',
                    liquidityTokenId: '5',
                    mgxTokenReserve: '496898413547286894785571735',
                    liquidityTokenReserve: '152919466150064409534841949',
                    collatorAccountAmountStakedInLp:
                      '3247617765048459965147037',
                    collatorAccountAmountStakedInMgx:
                      '10552849521963136690759768',
                    apyPerSessionInPercentage: '0.028309363920964778',
                  },
                },
                {
                  timestamp: '1692193602830',
                  data: {
                    timestamp: 1692193602830,
                    block: 2989219,
                    section: 'parachainStaking',
                    method: 'Rewarded',
                    sessionIndex: 2491,
                    collatorAccount:
                      '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
                    amountRewarded: '3049682978959055122100',
                    liquidityTokenId: '5',
                    mgxTokenReserve: '498146385622670707520541430',
                    liquidityTokenReserve: '153296294582212172810103656',
                    collatorAccountAmountStakedInLp:
                      '3247617765048459965147037',
                    collatorAccountAmountStakedInMgx:
                      '10553347397938914387586940',
                    apyPerSessionInPercentage: '0.028897778723314491',
                  },
                },
                {
                  timestamp: '1692208122954',
                  data: {
                    timestamp: 1692208122954,
                    block: 2990419,
                    section: 'parachainStaking',
                    method: 'Rewarded',
                    sessionIndex: 2492,
                    collatorAccount:
                      '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
                    amountRewarded: '2987444575204336243725',
                    liquidityTokenId: '5',
                    mgxTokenReserve: '497880205870442859369772655',
                    liquidityTokenReserve: '153296294582212172810103656',
                    collatorAccountAmountStakedInLp:
                      '3247617765048459965147037',
                    collatorAccountAmountStakedInMgx:
                      '10547708317787712454837064',
                    apyPerSessionInPercentage: '0.028323162578986883',
                  },
                },
              ]
              resolve(sessions || [])
            })
          }),
      }
    })
  })

  afterAll(async () => {
    vi.unmock('../src/repository/StakingRepository')
  })

  it('should mock the apy endpoint method', async () => {
    const expectedResponse: ResponseAPY[] = [
      {
        apy: '62.027726047981274135',
        token: '5',
        date: '16/08/2023',
        collatorAddress: '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac', //todo: qa to check what value we should set here
        dateFormat: 'DD/MM/YYYY',
        timestamp: '1692208122954',
      }
    ]
    const results = await apy(
      '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac'
    )
    expect(results).deep.equal(expectedResponse)
  })

  it('should mock the daily rewards endpoint method', async () => {
    const expectedResponse: DailyRewardResponse[] = [
      {
        tokenId: '5',
        dailyRewards: '17924667451226017462350',
        timestamp: 1692208122954,
        date: '16/08/2023',
        dateFormat: 'DD/MM/YYYY',
      },
    ]
    const results = await dailyRewards(
      '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
      undefined
    )
    expect(results).deep.equal(expectedResponse)
  })
})
