import { Response } from 'express'
import {
  AlreadyExistsException,
  BadRequestException,
  HttpResponseException,
  NotEnoughException,
  NotFoundException,
  ForbiddenException,
} from './Exception.js'
import logger from '../util/Logger.js'

export abstract class HttpStatusConstants {
  static readonly BAD_REQUEST: number = 400
  static readonly FORBIDDEN: number = 403
  static readonly NOT_FOUND: number = 404
  static readonly CONFLICT: number = 409
  static readonly RESOURCE_EXHAUSTED: number = 429
  static readonly UNEXPECTED: number = 500
}

export const handle = async (res: Response, error: Error): Promise<void> => {
  switch (true) {
    case error instanceof BadRequestException:
      logger.warn(error.message, error)
      res.status(HttpStatusConstants.BAD_REQUEST).json(errorToPayload(error))
      break
    case error instanceof NotFoundException:
      logger.debug(error.message, error)
      res.status(HttpStatusConstants.NOT_FOUND).json(errorToPayload(error))
      break
    case error instanceof AlreadyExistsException:
      logger.debug(error.message, error)
      res.status(HttpStatusConstants.CONFLICT).json(errorToPayload(error))
      break
    case error instanceof HttpResponseException:
      logger.debug(error.message, error)
      res.status(HttpStatusConstants.CONFLICT).json(errorToPayload(error))
      break
    case error instanceof NotEnoughException:
      logger.debug(error.message, error)
      res
        .status(HttpStatusConstants.RESOURCE_EXHAUSTED)
        .json(errorToPayload(error))
      break
    case error instanceof ForbiddenException:
      logger.debug(error.message, error)
      res.status(HttpStatusConstants.FORBIDDEN).json(errorToPayload(error))
      break
    default:
      logger.error(error.message, error)
      res.status(HttpStatusConstants.UNEXPECTED).json(errorToPayload(error))
      break
  }
}

export interface ErrorPayload {
  exceptionName: string
  message: string
}

export const errorToPayload = (error: Error): ErrorPayload => {
  return {
    exceptionName: error.constructor.name,
    message: error.message,
  }
}
