import { Request, Response } from 'express'
import * as networkService from '../service/NetworkService.js'
import * as errorHandler from '../error/Handler.js'

export const networkList = async (_: Request, res: Response) => {
  try {
    const affirmedNetworks = await networkService.listAffirmedNetworks()

    res.json(affirmedNetworks)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const tokenList = async (_: Request, res: Response) => {
  try {
    const affirmedTokens = await networkService.listAffirmedTokens()

    res.json(affirmedTokens)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
