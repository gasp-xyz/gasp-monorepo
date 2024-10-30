import MangataClient from '../connector/MangataNode.js'
import { BN } from '@polkadot/util'
import { ApiDecoration } from '@polkadot/api/types/index.js'
import { BN_ZERO } from 'gasp-sdk'
import { u128, u32 } from '@polkadot/types-codec'
import { signatureVerify } from '@polkadot/util-crypto'

const AIRDROP_WEIGHT = 0.5

export const getEligibilityAtBlockN = async (
  block: string,
  address: string
) => {
  const api = await (await MangataClient.api()).at(block)

  const balances = await getBalances(api, address)
  const issuance = await getTokenIssuance(api)

  const mgxPools = await getMgxPools(api)
  const poolMetadata = await getPoolMetadata(api, mgxPools)

  const lpAssets = Array.from(balances.keys()).filter((k) =>
    poolMetadata.has(k)
  )

  const mgxBalance = balances.get(process.env.MGX_TOKEN_ID) ?? BN_ZERO
  const mgxWeight = mgxBalance.muln(AIRDROP_WEIGHT * 10).divn(10)
  const lpMgxHoldings = lpAssets.map((lpId) => {
    return calculateTokenHoldings(
      poolMetadata.get(lpId),
      balances.get(lpId),
      issuance.get(lpId)
    )
  })

  const lpMgxHoldingsSum = lpMgxHoldings.reduce(
    (acc, weight) => acc.add(weight),
    BN_ZERO
  )
  const lpWeight = lpMgxHoldingsSum.muln(AIRDROP_WEIGHT * 10).divn(10)

  return {
    weight: mgxWeight.add(lpWeight),
    blockMgxHoldings: mgxBalance,
    blockMgxLpHoldings: lpMgxHoldingsSum,
    block,
  }
}

export function verifySignature(
  data: Record<'mangataXAddress' | 'ethereumAddress', string>,
  signature: string
) {
  const message = `0x${Buffer.from(JSON.stringify(data)).toString('hex')}`
  const { isValid } = signatureVerify(message, signature, data.mangataXAddress)

  return isValid
}

function calculateTokenHoldings(
  poolMetadata: Record<string, u128> | undefined,
  balance: BN,
  issuance: BN
) {
  if (!poolMetadata) {
    return BN_ZERO
  }

  const holdings = poolMetadata[process.env.MGX_TOKEN_ID]
    .mul(balance)
    .div(issuance)
    .muln(2)

  return holdings
}

async function getMgxPools(api: ApiDecoration<'promise'>) {
  const pools = await api.query.xyk.liquidityPools.entries()

  const mgxPools = pools
    .map<[string, [u32, u32]]>(([key, value]) => {
      return [key.args[0].toString(), value.unwrap()]
    })
    .filter(([, value]) => {
      return value.map(($) => $.toString()).includes(process.env.MGX_TOKEN_ID)
    })

  return mgxPools
}

const getBalances = async (api: ApiDecoration<'promise'>, address: string) => {
  return new Map(
    (await api.query.tokens.accounts.entries(address)).map(([key, value]) => {
      return [key.args[1].toString(), value.free.add(value.reserved)]
    })
  )
}

const getTokenIssuance = async (api: ApiDecoration<'promise'>) => {
  return new Map(
    (await api.query.tokens.totalIssuance.entries()).map(([key, value]) => {
      return [key.args[0].toString(), value.toBn()]
    })
  )
}

const getPoolMetadata = async (
  api: ApiDecoration<'promise'>,
  pools: Array<[string, [u32, u32]]>
) => {
  const poolsLiquidity = await Promise.all(
    pools.map(([, tokens]) => api.query.xyk.pools(tokens))
  )

  const lpAssets = await Promise.all(
    pools.map(([, tokens]) => api.query.xyk.liquidityAssets(tokens))
  )

  const poolsArray = pools.map<[string, Record<string, u128>]>(
    ([, tokens], i) => {
      return [
        lpAssets[i].toString(),
        {
          [tokens[0].toString()]: poolsLiquidity[i][0],
          [tokens[1].toString()]: poolsLiquidity[i][1],
        },
      ]
    }
  )

  return new Map(poolsArray)
}
