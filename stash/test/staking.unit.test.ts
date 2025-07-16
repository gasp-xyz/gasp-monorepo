import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest'

import {
  DailyRewardResponse,
  ResponseAPY,
  Session,
} from '../src/repository/StakingRepository'
import { apy, dailyRewards } from '../src/service/StakingAprService'

describe('[Staking]', () => {
  beforeAll(() => {
    vi.mock('../src/repository/StakingRepository', async () => {
      const actual = await vi.importActual(
        '../src/repository/StakingRepository',
      )
      return {
        ...actual,
        getDataByAddress: vi.fn().mockImplementation(() => {
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
                  collatorAccount: '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
                  amountRewarded: '2987444575204336243725',
                  liquidityTokenId: '5',
                  mgxTokenReserve: '496898413547286894785571735',
                  liquidityTokenReserve: '152919466150064409534841949',
                  collatorAccountAmountStakedInLp: '3247617765048459965147037',
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
                  collatorAccount: '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
                  amountRewarded: '3049682978959055122100',
                  liquidityTokenId: '5',
                  mgxTokenReserve: '498146385622670707520541430',
                  liquidityTokenReserve: '153296294582212172810103656',
                  collatorAccountAmountStakedInLp: '3247617765048459965147037',
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
                  collatorAccount: '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
                  amountRewarded: '2987444575204336243725',
                  liquidityTokenId: '5',
                  mgxTokenReserve: '497880205870442859369772655',
                  liquidityTokenReserve: '153296294582212172810103656',
                  collatorAccountAmountStakedInLp: '3247617765048459965147037',
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
        apy: '212.550402589504005115',
        token: '0',
        date: '19/11/2024',
        collatorAddress: '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
        dateFormat: 'DD/MM/YYYY',
        timestamp: '1732018266000',
      },
    ]
    const results = await apy('0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac')
    expect(results).deep.equal(expectedResponse)
  })

  it('should mock the daily rewards endpoint method', async () => {
    const expectedResponse: DailyRewardResponse[] = [
      {
        tokenId: '0',
        dailyRewards: '582330180078676136073516',
        timestamp: 1732018266000,
        date: '19/11/2024',
        dateFormat: 'DD/MM/YYYY',
      },
    ]
    const results = await dailyRewards(
      '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac',
      undefined,
    )
    expect(results).deep.equal(expectedResponse)
  })
})
