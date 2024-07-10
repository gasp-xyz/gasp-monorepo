import { Decimal } from 'decimal.js'

export const ZERO = new Decimal(0)
export const API_LIMIT = 50_000

export const partition = <T>(arr: T[], fn: Predicate<T>) =>
  arr.reduce(
    (acc, val, i, arr) => {
      acc[fn(val) ? 0 : 1].push(val)
      return acc
    },
    [[], []] as [T[], T[]]
  )

export type Predicate<T> = (asset: T) => boolean
