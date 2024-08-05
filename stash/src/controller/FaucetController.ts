import * as faucetService from '../service/FaucetService.js'
import * as errorHandler from '../error/Handler.js'
import { Request, Response } from 'express'
import { pathFaucetSchema } from '../schema/FaucetSchema.js'

export const captcha = async (req: Request, res: Response) => {
  /*
  #swagger.tags = ['Faucet']
  #swagger.summary = 'Request tokens from the faucet.'
  #swagger.description = "Request tokens from the faucet by providing the recipient address and passing captcha verification."
  #swagger.parameters['toAddress'] = {
    in: 'path',
    description: 'The address to receive the tokens',
    required: true,
    type: 'string'
  }
  #swagger.parameters['captcha'] = {
    in: 'path',
    description: 'Captcha token to verify the request',
    required: true,
    type: 'string'
  }
  #swagger.responses[200] = {
     description: 'Successful response with no content'
  }
*/
  try {
    const params = pathFaucetSchema.validateSync(req.params)
    await faucetService.verifyCaptcha(params.captcha)
    await faucetService.sendTokens(params.toAddress)
    res.status(200).send()
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
