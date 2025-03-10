import { Schema } from 'redis-om'

const tokenPricesSchema = new Schema(
  'token_prices',
  {
    tokenId: { type: 'string' },
    chainId: { type: 'string' },
    contractAddress: { type: 'string' },
    price: { type: 'number' },
    timestamp: { type: 'date' },
    created: { type: 'date' },
    updated: { type: 'date' },
  },
  {
    dataStructure: 'JSON',
  }
)

export { tokenPricesSchema }
