import { Request, Response } from 'express'
import * as liquidityStakingService from '../service/LiquidityStakingService.js'
import * as errorHandler from '../error/Handler.js'

export const rewards24hours = async (req: Request, res: Response) => {
  try {
    const data = await liquidityStakingService.rewards24hours(
      req.params.address
    )
    res.send(data)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const rewardsMonth = async (req: Request, res: Response) => {
  try {
    const data = await liquidityStakingService.rewardsMonth(req.params.address)
    res.send(data)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
