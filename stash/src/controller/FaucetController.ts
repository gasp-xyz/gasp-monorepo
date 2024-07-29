import * as faucetService from '../service/FaucetService.js'
import * as errorHandler from '../error/Handler.js'
import { Request, Response } from 'express'

export const captcha = async (req: Request, res: Response) => {
  try {
    const data = req.params
    const address = data.address
    const captchaToken = data.captchaToken
    console.log('data', address, captchaToken)
    const verifiedCaptcha = await faucetService.verifyCaptcha(captchaToken)
    if (verifiedCaptcha) {
      console.log('Captcha ok, requesting tokens from ', address)
      //todo: request tokens from address
    } else {
      console.log('Captcha failed')
    }
    res.send(verifiedCaptcha)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
