import { ApiPromise, Keyring } from '@polkadot/api'
import { BN, u8aToHex } from '@polkadot/util'
import { toBN } from 'gasp-sdk'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import {
  getEligibilityAtBlockN,
  verifySignature,
} from '../src/service/MgxAirdropService.js'

vi.mock('../src/repository/MgxAirdropRepository')

describe('MGX Airdrop', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  afterEach(() => {
    vi.resetAllMocks()
  })

  it('calculate correct weight for MGX balance', async () => {
    const api = (await provideApiMock({
      tokens: [['0', ['1', '2']]],
    })) as unknown as ApiPromise
    const { weight, blockMgxLpHoldings, blockMgxHoldings } =
      await getEligibilityAtBlockN(api, '123', 'address')

    expect(weight.toString()).toEqual(toBN('0.010339800000000000').toString())
    expect(blockMgxHoldings.toString()).toEqual(toBN('3').toString())
    expect(blockMgxLpHoldings.toString()).toEqual(toBN('0').toString())
  })

  it('calculate correct weight for LP tokens', async () => {
    const api = (await provideApiMock({
      totalIssuance: [
        ['0', '200'],
        ['1', '100'],
      ],
      tokens: [
        ['0', ['0', '0']],
        ['1', ['0', '10']],
      ],
      liquidityAssets: [[['0', '2'], '1']],
      liquidityPools: [['1', ['0', '2']]],
      pools: [
        [
          ['0', '2'],
          [toBN('10'), toBN('8')],
        ],
      ],
    })) as unknown as ApiPromise

    const { weight, blockMgxHoldings, blockMgxLpHoldings } =
      await getEligibilityAtBlockN(api, '123', 'address')

    expect(weight.toString()).toEqual(toBN('0.006893200000000000').toString())
    expect(blockMgxHoldings.toString()).toEqual(toBN('0').toString())
    expect(blockMgxLpHoldings.toString()).toEqual(toBN('2').toString())
  })

  it('calculate correct weight for both MGX and multiple LP tokens', async () => {
    const api = (await provideApiMock({
      totalIssuance: [
        ['0', '200'],
        ['1', '100'],
        ['3', '100'],
      ],
      tokens: [
        ['0', ['3', '0']],
        ['1', ['0', '10']],
        ['3', ['0', '10']],
      ],
      liquidityAssets: [
        [['0', '2'], '1'],
        [['4', '0'], '3'],
      ],
      liquidityPools: [
        ['1', ['0', '2']],
        ['3', ['4', '0']],
      ],
      pools: [
        [
          ['0', '2'],
          [toBN('10'), toBN('8')],
        ],
        [
          ['4', '0'],
          [toBN('8'), toBN('10')],
        ],
      ],
    })) as unknown as ApiPromise

    const { weight, blockMgxHoldings, blockMgxLpHoldings } =
      await getEligibilityAtBlockN(api, '123', 'address')

    expect(weight.toString()).toEqual(toBN('0.024126200000000000').toString())
    expect(blockMgxHoldings.toString()).toEqual(toBN('3').toString())
    expect(blockMgxLpHoldings.toString()).toEqual(toBN('4').toString())
  })

  it('verify valid signature', async () => {
    const keyring = new Keyring({ type: 'sr25519' })
    const pair = keyring.addFromUri('//Alice')

    const data = {
      mangataXAddress: pair.address,
      ethereumAddress: 'ethereum',
    }
    const message = `0x${Buffer.from(JSON.stringify(data)).toString('hex')}`

    const signature = u8aToHex(pair.sign(message))

    const isValid = verifySignature(data, signature)

    expect(isValid).toBeTruthy()
  })

  it('verify invalid signature', async () => {
    const keyring = new Keyring({ type: 'sr25519' })
    const pair = keyring.addFromUri('//Alice')

    const signedData = {
      mangataXAddress: pair.address,
      ethereumAddress: 'ethereum',
    }

    const verifiedData = {
      mangataXAddress: pair.address,
      ethereumAddress: 'eth',
    }
    const message = `0x${Buffer.from(JSON.stringify(signedData)).toString(
      'hex'
    )}`

    const signature = u8aToHex(pair.sign(message))

    const isValid = verifySignature(verifiedData, signature)

    expect(isValid).toBeFalsy()
  })
})

function provideTokensMock(tokens: Array<[string, [string, string]]>) {
  return vi.fn().mockResolvedValue(
    tokens.map(([id, [free, reserved]]) => {
      return [
        {
          args: ['', id],
        },
        {
          free: toBN(free),
          reserved: toBN(reserved),
        },
      ]
    })
  )
}

function provideTotalIssuanceMock(totalIssuance: Array<[string, string]>) {
  return vi.fn().mockResolvedValue(
    totalIssuance.map(([id, totalIssuance]) => {
      return [
        {
          args: [id],
        },
        {
          toBn: () => toBN(totalIssuance),
        },
      ]
    })
  )
}

function provideLiquidityPoolsMock(pools: Array<[string, [string, string]]>) {
  return vi.fn().mockResolvedValue(
    pools.map(([id, [token1, token2]]) => {
      return [
        {
          args: [id],
        },
        {
          unwrap: () => [new BN(token1), new BN(token2)],
        },
      ]
    })
  )
}

function provideLiquidityAssetsMock(assets: Array<[[string, string], string]>) {
  return vi.fn().mockImplementation((tokens) => {
    const asset = assets.find(([[token1, token2]]) => {
      return tokens[0].toString() === token1 && tokens[1].toString() === token2
    })
    return asset?.[1]
  })
}

function providePoolsMock(pools: Array<[[string, string], [BN, BN]]>) {
  return vi.fn().mockImplementation((tokens) => {
    const pool = pools.find(([[token1, token2]]) => {
      return tokens[0].toString() === token1 && tokens[1].toString() === token2
    })
    return pool?.[1]
  })
}

async function provideApiMock({
  tokens,
  totalIssuance,
  liquidityPools,
  liquidityAssets,
  pools,
}: {
  tokens?: Array<[string, [string, string]]>
  totalIssuance?: Array<[string, string]>
  liquidityPools?: Array<[string, [string, string]]>
  liquidityAssets?: Array<[[string, string], string]>
  pools?: Array<[[string, string], [BN, BN]]>
}) {
  const api = {
    query: {
      tokens: {
        accounts: {
          entries: provideTokensMock(tokens ?? []),
        },
        totalIssuance: {
          entries: provideTotalIssuanceMock(totalIssuance ?? []),
        },
      },
      xyk: {
        pools: providePoolsMock(pools ?? []),
        liquidityAssets: provideLiquidityAssetsMock(liquidityAssets ?? []),
        liquidityPools: {
          entries: provideLiquidityPoolsMock(liquidityPools ?? []),
        },
      },
    },
  }

  return {
    at: vi.fn().mockResolvedValue(api),
  }
}
