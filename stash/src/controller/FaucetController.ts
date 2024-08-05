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
     description: 'The address to receive the tokens (must begin with 0x)',
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
   #swagger.responses[403] = {
      description: 'Forbidden',
      content: {
          'application/json': {
              schema: {
                  oneOf: [
                      {
                          example: {
                              "message": "Captcha verification failed. Reason: [error-codes]"
                          }
                      },
                      {
                          example: {
                              "message": "Address [toAddress] has requested the token more than 3 times."
                          }
                      }
                  ]
              }
          }
      }
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
