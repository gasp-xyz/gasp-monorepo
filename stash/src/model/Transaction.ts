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
  },
  {
    dataStructure: 'JSON',
  }
)

export { transactionSchema }
