import { Decimal } from 'decimal.js'

export type TimestampedAmount = [number, Decimal]
export type TimestampedAmountPool = [number, [Decimal, Decimal]]
export type TimestampedBaseTargetAmount = [number, number, number, Decimal]

export const ASSET_ALL = {
  id: 'ALL',
  chainId: 0,
  name: 'Total Aggregate',
  symbol: 'ALL',
  address: '',
  decimals: 0,
}
