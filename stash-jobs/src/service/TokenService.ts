import * as tokenRepository from '../repository/TokenRepository.js'
import { TokenOrderBucket } from '../repository/TokenRepository.js'

import 'core-js/proposals/array-grouping-stage-3-2.js'

export const initService = async (): Promise<void> => {
  await tokenRepository.initData()
}

export const listTokenOrderBuckets = async (): Promise<TokenOrderBucket[]> => {
  const tokens = await tokenRepository.listTokenOrderBuckets()
  const sortedResult = Array.from(tokens.values()).sort(
    (a, b) => a.rank - b.rank
  )

  return [...sortedResult]
}
