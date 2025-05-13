import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import { keyValueSchema } from '../schema/KeyValueSchema.js'
import * as keyValueService from '../service/KeyValueService.js'

export const store = async (req: Request, res: Response): Promise<void> => {
  try {
    const { key, value } = req.body
    keyValueSchema.validateSync({ key, value })
    const stored = await keyValueService.store(key, value)
    res.json({ stored: stored })
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
