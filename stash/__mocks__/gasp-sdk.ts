import { vi } from 'vitest'
import { BN } from '@polkadot/util'
import { fromBN } from 'gasp-sdk'

const CUSTOM_ISSUANCE = {
  '0': new BN(100), // Custom issuance for ID 0
  '1': new BN(200), // Custom issuance for ID 4
  '5': new BN(100), // Custom issuance for ID 0
  '6': new BN(100), // Custom issuance for ID 0
  '7': new BN(1000), // Custom issuance for ID 4
  '8': new BN(200), // Custom issuance for ID 4
}

const ASSETS_INFO = {
  '0': {
    id: '0',
    chainId: 0,
    decimals: 18,
    name: 'Gasp Ethereum',
    symbol: 'GETH',
    address: '',
  },
  '1': {
    id: '1',
    chainId: 0,
    decimals: 18,
    name: 'Gasp V2',
    symbol: 'GASPV2',
    address: '',
  },
  '2': { id: '2', decimals: 18, name: 'L1Asset', symbol: 'L1Asset' },
  '3': { id: '3', decimals: 18, name: 'L1Asset', symbol: 'L1Asset' },
  '4': { id: '4', decimals: 18, name: 'L1Asset', symbol: 'L1Asset' },
  '5': {
    id: '5',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'GASPV2-GETH',
    address: '',
  },
  '6': {
    id: '6',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'L1Asset-GETH',
    address: '',
  },
  '7': {
    id: '7',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'L1Asset-GASPV2',
    address: '',
  },
  '8': {
    id: '8',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'GASPV2-L1Asset',
    address: '',
  },
}

const tokenAmountMapping = {
  '1-0': [new BN('11788919631539815'), new BN('486263164987593456634092965')],
}

const mockMangata = (urls: string[]) => ({
  query: {
    getAssetsInfo: () => {
      return new Promise((resolve) => {
        resolve(ASSETS_INFO)
      })
    },

    getTotalIssuance: (id: string) => {
      return CUSTOM_ISSUANCE[id] !== undefined ? CUSTOM_ISSUANCE[id] : new BN(0)
    },

    getAmountOfTokensInPool: (baseTokenId: string, targetTokenId: string) => {
      const key = `${baseTokenId}-${targetTokenId}`
      const defaultValues = [new BN(0), new BN(0)]

      // Use the mapping to get the corresponding values or use default values if not found
      return tokenAmountMapping[key] || defaultValues
    },
  },
  rpc: {
    calculateBuyPrice: (args: {
      inputReserve: BN
      outputReserve: BN
      amount: BN
    }) => {
      const mappingKeys = Object.keys(tokenAmountMapping)

      for (const key of mappingKeys) {
        const [mappingBaseReserve, mappingTargetReserve] =
          tokenAmountMapping[key]

        if (
          args.inputReserve.eq(mappingTargetReserve) &&
          args.outputReserve.eq(mappingBaseReserve)
        ) {
          const mockPrices = {
            '1-0': new BN('41375099191776760995251'),
          }

          return mockPrices[key]
        }
      }

      return new BN(0)
    },
  },
})

type Reserve = {
  inputReserve: BN
  outputReserve: BN
  amount: BN
}

const Mangata = {
  instance: vi.fn((urls) => mockMangata(urls)),
}

export { fromBN, Mangata }
