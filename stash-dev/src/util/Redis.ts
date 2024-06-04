import { ChainableCommander } from 'ioredis'
import _ from 'lodash'
import { RedisClient } from '../connector/RedisConnector.js'
import logger from './Logger.js'

export type SaveEntry = [string, number, string]
export type CreateTableArgs = [string, (string | number)[]][]

export const execute = async (trx: ChainableCommander) => {
  const r = await trx.exec()
  const result = r
    .filter(([err, res]) => _.isArray(res))
    .map(([err, res]) => res)
    .flat()
  const err = result.find((r) => !_.isNumber(r))
  if (err) {
    const ops = trx['_queue']
      .filter((c) => c.name === 'TS.MADD')
      .map((c) => _.chunk(c.args, 3))
      .flat()
    logger.error(`error at ${ops[result.indexOf(err)]} with ${err}`)
    throw Error()
  }
  return result.length
}

export const hasKey = async (
  client: RedisClient,
  key: string
): Promise<boolean> => {
  return (await client.client.exists(key)) > 0
}
