import { Schema } from 'redis-om'

const transactionSchema = new Schema(
  'transaction',
  {
    address: { type: 'string' },
    txHash: { type: 'string' },
    requestId: { type: 'string' },
    status: { type: 'string' },
    created: { type: 'number' },
    updated: { type: 'number' },
    type: { type: 'string' },
    amount: { type: 'number' },
    asset_chainId: { type: 'string' },
    asset_address: { type: 'string' },
  },
  {
    dataStructure: 'JSON',
  }
)

export { transactionSchema }
