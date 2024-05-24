import { vi } from 'vitest'
import { BN } from '@polkadot/util'
import { fromBN } from '@mangata-finance/sdk'

const CUSTOM_ISSUANCE = {
  '0': new BN(100), // Custom issuance for ID 0
  '4': new BN(200), // Custom issuance for ID 4
  '5': new BN(100), // Custom issuance for ID 0
  '7': new BN(200), // Custom issuance for ID 4
  '8': new BN(100), // Custom issuance for ID 0
  '9': new BN(1000), // Custom issuance for ID 4
}

const ASSETS_INFO = {
  '0': {
    id: '0',
    chainId: 0,
    decimals: 18,
    name: 'Mangata',
    symbol: 'MGX',
    address: '',
  },
  '4': {
    id: '4',
    chainId: 0,
    decimals: 12,
    name: 'Kusama Native',
    symbol: 'KSM',
    address: '',
  },
  '5': {
    id: '5',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'KSM-MGX',
    address: '',
  },
  '7': {
    id: '7',
    chainId: 0,
    decimals: 10,
    name: 'Turing native token',
    symbol: 'TUR',
    address: '',
  },
  '8': {
    id: '8',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'MGX-TUR',
    address: '',
  },
  '9': {
    id: '9',
    chainId: 0,
    decimals: 18,
    name: 'Liquidity Pool Token',
    symbol: 'KSM-TUR',
    address: '',
  },
}

const tokenAmountMapping = {
  '4-0': [new BN('11788919631539815'), new BN('486263164987593456634092965')],
  '0-7': [new BN('54518136758845743088272528'), new BN('75865507392016543')],
  '4-7': [new BN('2941582428288'), new BN('168424720224156')],
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
    calculateBuyPrice: (args: { inputReserve: BN; outputReserve: BN; amount: BN }) => {
      const mappingKeys = Object.keys(tokenAmountMapping)

      for (const key of mappingKeys) {
        const [mappingBaseReserve, mappingTargetReserve] = tokenAmountMapping[key]

        if (args.inputReserve.eq(mappingTargetReserve) && args.outputReserve.eq(mappingBaseReserve)) {
          const mockPrices = {
            '4-0': new BN('41375099191776760995251'),
            '0-7': new BN('1395751849'),
            '4-7': new BN('87007130012718'),
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
