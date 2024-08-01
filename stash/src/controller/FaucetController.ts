import * as faucetService from '../service/FaucetService.js'
import * as errorHandler from '../error/Handler.js'
import { Request, Response } from 'express'
import { pathFaucetSchema } from '../schema/FaucetSchema.js'

export const captcha = async (req: Request, res: Response) => {
  try {
    const params = pathFaucetSchema.validateSync(req.params)
    await faucetService.verifyCaptcha(params.captcha)
    await faucetService.sendTokens(params.toAddress)
    res.status(200).send()
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
