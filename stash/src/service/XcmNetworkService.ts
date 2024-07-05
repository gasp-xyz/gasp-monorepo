import * as xcmNetworkRepository from '../repository/XcmNetworkRepository.js'
import {
  XcmChannel,
  XcmChannelToken,
} from '../repository/XcmNetworkRepository.js'

import 'core-js/proposals/array-grouping-stage-3-2.js'

export const initService = async (): Promise<void> => {
  await xcmNetworkRepository.initNetwork()
}

export const listFullNetwork = async (): Promise<XcmChannelNetwork> => {
  const channels = await xcmNetworkRepository.listAllChannels()
  const tokens = await xcmNetworkRepository.listAllTokens()

  return {
    channels: [...channels.values()],
    tokens: [...tokens.values()],
  } as XcmChannelNetwork
}

export const listTokens = async (): Promise<XcmChannelToken[]> => {
  const tokens = await xcmNetworkRepository.listAllTokens()

  return [...tokens.values()]
}

export const listChannels = async (): Promise<XcmChannel[]> => {
  const channels = await xcmNetworkRepository.listAllChannels()

  return [...channels.values()]
}

interface XcmChannelNetwork {
  channels: XcmChannel[]
  tokens: XcmChannelToken[]
}
