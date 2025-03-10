import { Schema } from 'redis-om'

const swapSchema = new Schema(
  'swaps',
  {
    account: { type: 'string' },
    daysActive: { type: 'number' },
    lastTradeTimestamp: { type: 'date' },
    totalVolumes: { type: 'number' },
    totalTrades: { type: 'number' },
  },
  {
    dataStructure: 'JSON',
  }
)

export { swapSchema }
