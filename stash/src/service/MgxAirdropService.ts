import { ApiPromise } from '@polkadot/api'
import { ApiDecoration } from '@polkadot/api/types/index.js'
import { u32, u128 } from '@polkadot/types-codec'
import { BN } from '@polkadot/util'
import { signatureVerify } from '@polkadot/util-crypto'
import { BN_ZERO } from 'gasp-sdk'

const AIRDROP_WEIGHT_SCALE_FACTOR = new BN(10000000)
const AIRDROP_WEIGHT = new BN(34466)

export const getEligibilityAtBlockN = async (
  _api: ApiPromise,
  block: string,
  address: string,
) => {
  const api = await _api.at(block)

  const balances = await getBalances(api, address)
  const issuance = await getTokenIssuance(api)

  const mgxPools = await getMgxPools(api)
  const poolMetadata = await getPoolMetadata(api, mgxPools)

  const lpAssets = Array.from(balances.keys()).filter((k) =>
    poolMetadata.has(k),
  )

  const mgxBalance = balances.get(process.env.MGX_TOKEN_ID) ?? BN_ZERO
  const mgxWeight = mgxBalance
    .mul(AIRDROP_WEIGHT)
    .div(AIRDROP_WEIGHT_SCALE_FACTOR)
  const lpMgxHoldings = lpAssets.map((lpId) => {
    return calculateTokenHoldings(
      poolMetadata.get(lpId),
      balances.get(lpId),
      issuance.get(lpId),
    )
  })

  const lpMgxHoldingsSum = lpMgxHoldings.reduce(
    (acc, weight) => acc.add(weight),
    BN_ZERO,
  )
  const lpWeight = lpMgxHoldingsSum
    .mul(AIRDROP_WEIGHT)
    .div(AIRDROP_WEIGHT_SCALE_FACTOR)

  return {
    weight: mgxWeight.add(lpWeight),
    blockMgxHoldings: mgxBalance,
    blockMgxLpHoldings: lpMgxHoldingsSum,
    block,
  }
}

export function verifySignature(
  data: Record<'mangataXAddress' | 'ethereumAddress', string>,
  signature: string,
) {
  const message = `0x${Buffer.from(JSON.stringify(data)).toString('hex')}`
  const { isValid } = signatureVerify(message, signature, data.mangataXAddress)

  return isValid
}

function calculateTokenHoldings(
  poolMetadata: Record<string, u128> | undefined,
  balance: BN,
  issuance: BN,
) {
  if (!poolMetadata) {
    return BN_ZERO
  }

  const mgxValue = poolMetadata[process.env.MGX_TOKEN_ID]
  const mgxBN = typeof mgxValue.toBn === 'function' ? mgxValue.toBn() : mgxValue
  return mgxBN.mul(balance).div(issuance).muln(2)
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
      return [
        key.args[1].toString(),
        new BN(value.free.toString()).add(new BN(value.reserved.toString())),
      ]
    }),
  )
}

const getTokenIssuance = async (api: ApiDecoration<'promise'>) => {
  return new Map(
    (await api.query.tokens.totalIssuance.entries()).map(([key, value]) => {
      return [key.args[0].toString(), value.toBn()]
    }),
  )
}

const getPoolMetadata = async (
  api: ApiDecoration<'promise'>,
  pools: Array<[string, [u32, u32]]>,
) => {
  const poolsLiquidity = await Promise.all(
    pools.map(([, tokens]) => api.query.xyk.pools(tokens)),
  )

  const lpAssets = await Promise.all(
    pools.map(([, tokens]) => api.query.xyk.liquidityAssets(tokens)),
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
    },
  )

  return new Map(poolsArray)
}
