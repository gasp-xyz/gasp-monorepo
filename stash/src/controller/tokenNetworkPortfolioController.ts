import { Request, Response } from 'express'
import * as errorHandler from '../error/Handler.js'
import MangataClient from '../connector/MangataNode.js'
import { BN } from '@polkadot/util'
import {
  priceDiscovery,
  priceHistory,
} from '../service/PriceDiscoveryService.js'
import { Decimal } from 'decimal.js'
import { fromBN } from 'gasp-sdk'
import logger from '../util/Logger.js'

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
        const frozen = JSON.parse(
          JSON.stringify(value.toHuman())
        ).frozen.toString()
        const frozenWithoutCommas = frozen.replace(/,/g, '')
        const freeTokens = new BN(freeWithoutCommas)
        const frozenTokens = new BN(frozenWithoutCommas)
        const freeBalance = freeTokens.sub(frozenTokens)
        const tokenId = storageKey.args[1].toString()
        let tokenBalanceInUsd: string
        let tokenInfo: any
        let balanceInUsdCalculated: any

        try {
          const assetsInfo = await MangataClient.query.getAssetsInfo()
          tokenInfo = Object.values(assetsInfo).find(
            (item) => item.id === tokenId
          )
        } catch (error) {
          logger.error(
            `Error fetching token info for tokenId ${tokenId}:`,
            error
          )
          tokenInfo = null
        }

        try {
          tokenBalanceInUsd = (await priceDiscovery(tokenId)).current_price[
            'usd'
          ]
          if (tokenBalanceInUsd === '0') {
            throw new Error(
              `Token balance in USD is zero for tokenId ${tokenId}`
            )
          }
        } catch (error) {
          logger.error(
            'Error fetching token balance in USD, fallback option price-history:',
            error
          )
          try {
            // only: no price-discovery for token and then only for tokens that are not pools and not LP tokens, price-history will fail for pools/LP tokens
            const priceHistoryData = await priceHistory(tokenId, 1, 0) // 1 day in history max, interval 0 to not aggreagate results
            tokenBalanceInUsd =
              priceHistoryData.prices.length > 0
                ? priceHistoryData.prices[
                    priceHistoryData.prices.length - 1
                  ][1].toString()
                : '0.00' //if there is no price-history data fallback to 0.00
          } catch (error) {
            logger.error(
              `Error fetching token info and balanceInUSD for tokenId ${tokenId}, it might be a pool or a LP token:`,
              error
            )
            tokenBalanceInUsd = '0.00' //no price-discovery + its pool/LP token
          }
        }

        new Decimal(freeBalance.toString()).mul(new Decimal(tokenBalanceInUsd))
        if (tokenInfo && tokenInfo.decimals) {
          //we pass decimals to fromBN if its a token
          balanceInUsdCalculated = new Decimal(
            fromBN(freeBalance, tokenInfo.decimals ?? 18)
          )
            .mul(new Decimal(tokenBalanceInUsd))
            .toFixed(2)
            .toString()
        } else {
          balanceInUsdCalculated = new Decimal(0.0) //no value
        }
        logger.info(
          'FINAL: freeBalance:',
          freeBalance.toString(),
          'tokenBalanceInUsd:',
          tokenBalanceInUsd,
          'tokenInfo.decimals:',
          tokenInfo?.decimals,
          'tokenId:',
          tokenId,
          'balanceInUsdCalculated:',
          balanceInUsdCalculated
        )
        return {
          tokenId,
          balance: freeBalance.toString(),
          balanceInUsd: balanceInUsdCalculated,
        }
      }
    )
    const results = await Promise.all(portfolioBalance)
    res.json(results)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
