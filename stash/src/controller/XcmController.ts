import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import * as xcmService from '../service/XcmNetworkService.js'

export const network = async (req: Request, res: Response): Promise<void> => {
  try {
    const fullNetwork = await xcmService.listFullNetwork()

    res.json(fullNetwork)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const channels = async (req: Request, res: Response): Promise<void> => {
  try {
    const channels = await xcmService.listChannels()

    res.json(channels)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const tokens = async (req: Request, res: Response): Promise<void> => {
  try {
    const tokens = await xcmService.listTokens()

    res.json(tokens)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
