import { Schema } from 'redis-om'

const depositSchema = new Schema(
  'deposits',
  {
    address: { type: 'string' },
    recipient: { type: 'string' },
    txHash: { type: 'string' },
    requestId: { type: 'number' },
    status: { type: 'string' },
    created: { type: 'number' },
    updated: { type: 'number' },
    type: { type: 'string' },
    chain: { type: 'string' },
    amount: { type: 'string' },
    asset_chainId: { type: 'string' },
    asset_address: { type: 'string' },
    createdBy: { type: 'string' },
    closedBy: { type: 'string' },
  },
  {
    dataStructure: 'JSON',
  },
)

export { depositSchema }
