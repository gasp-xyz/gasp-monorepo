import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import { dailyRewardDateSchema } from '../schema/StakingSchema.js'
import * as stakingAprService from '../service/StakingAprService.js'

export const dailyReward = async (req: Request, res: Response) => {
  try {
    const { date } = dailyRewardDateSchema.validateSync(req.query)
    const dailyRewards = await stakingAprService.dailyRewards(
      req.params.collatorAddress,
      date,
    )
    res.json(dailyRewards)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const apy = async (req: Request, res: Response) => {
  try {
    const apy = await stakingAprService.apy(req.params.collatorAddress)
    res.json(apy)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const collatorsApy = async (req: Request, res: Response) => {
  try {
    const apy = await stakingAprService.collatorsApy()
    res.json(apy)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
