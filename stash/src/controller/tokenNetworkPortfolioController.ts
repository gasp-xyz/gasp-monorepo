import { BN } from '@polkadot/util'
import { Decimal } from 'decimal.js'
import { Request, Response } from 'express'
import { fromBN } from 'gasp-sdk'

import MangataClient from '../connector/MangataNode.js'
import * as errorHandler from '../error/Handler.js'
import { getTokenPrice } from '../service/TokenPriceService.js'
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
          tokenBalanceInUsd = (await getTokenPrice(tokenId)).toString()
        } catch (error) {
          logger.error(
            `Error fetching token info and balanceInUSD for tokenId ${tokenId}, it might be a pool or a LP token:`,
            error
          )
          tokenBalanceInUsd = '0.00' //no price, default to 0
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
