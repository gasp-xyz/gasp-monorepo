import { Entity, Schema } from 'redis-om'

export interface MgxAirdropSchema extends Entity {
  mangataXAddress: string
  isEligible: boolean
  ethereumAddress: string
  blockMgxHoldings: string
  blockMgxLpHoldings: string
  weight: string
}

const mgxAirdropSchema = new Schema<MgxAirdropSchema>(
  'mgx-airdrop',
  {
    mangataXAddress: { type: 'string' },
    isEligible: { type: 'boolean' },
    ethereumAddress: { type: 'string' },
    blockMgxHoldings: { type: 'string' },
    blockMgxLpHoldings: { type: 'string' },
    weight: { type: 'string' },
  },
  {
    dataStructure: 'JSON',
  },
)

export { mgxAirdropSchema }
