import { Request, Response } from 'express'
import * as errorHandler from '../error/Handler.js'
import MangataClient from '../connector/MangataNode.js'
import { BN } from '@polkadot/util'
import { priceDiscovery, getAsset } from '../service/PriceDiscoveryService.js'
import { Decimal } from 'decimal.js'
import { fromBN } from 'gasp-sdk'

export const tokenNetworkPortfolio = async (req: Request, res: Response) => {
  try {
    const api = await MangataClient.api()
    const accountBalances = await api.query.tokens.accounts.entries(
      req.params.address
    )
    const portfolioBalance = accountBalances.map(
      async ([storageKey, value]) => {
        const free = JSON.parse(JSON.stringify(value.toHuman())).free.toString()
        const freeWithoutCommas = free.replace(/,/g, '')
        const frozen = JSON.parse(JSON.stringify(value)).frozen.toString()
        const freeTokens = new BN(freeWithoutCommas)
        const frozenTokens = new BN(frozen)
        const freeBalance = freeTokens.sub(frozenTokens)
        const tokenId = storageKey.args[1].toString()
        console.log('storage key', storageKey.args[1].toString())
        const tokenInfo = (
          await api.query.assetRegistry.metadata(tokenId)
        ).toHuman() as {
          symbol: string
        }

        let tokenBalanceInUsd: string
        try {
          tokenBalanceInUsd = (await priceDiscovery(tokenInfo.symbol))
            .current_price['usd']
        } catch (error) {
          console.error('Error fetching token balance in USD:', error)
          tokenBalanceInUsd = '0'
        }

        new Decimal(freeBalance.toString()).mul(new Decimal(tokenBalanceInUsd))
        return {
          tokenId,
          balance: freeBalance.toString(),
          balanceInUsd: new Decimal(fromBN(freeBalance))
            .mul(new Decimal(tokenBalanceInUsd))
            .toFixed(2)
            .toString(),
        }
      }
    )
    const results = await Promise.all(portfolioBalance)
    res.json(results)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
