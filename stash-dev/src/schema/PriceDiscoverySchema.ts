import { lazy, mixed, number, object, string } from 'yup'

export type Interval =
  | 'day'
  | '12H'
  | '6H'
  | 'hour'
  | '30m'
  | '15m'
  | 'minute'
  | 'block'
const interval: Interval[] = [
  'day',
  '12H',
  '6H',
  'hour',
  '30m',
  '15m',
  'minute',
  'block',
]

export const bodyHistorySchema = object({
  days: lazy((d) =>
    isNaN(d) ? mixed<'max'>().oneOf(['max']) : number().positive()
  ),
  interval: lazy((d) =>
    isNaN(d) ? mixed<Interval>().oneOf(interval) : number().positive()
  ),
})

export const pathCurrencySchema = object({
  currencySymbol: string().required(),
})
export const pathPairCurrencySchema = object({
  baseCurrencySymbol: string().required(),
  targetCurrencySymbol: string().required(),
})

export const tokenSymbolSchema = object({
  symbol: string()
    .required()
    .test('no-hyphen', 'Token symbol cannot contain a hyphen (-)', (value) => {
      if (typeof value === 'string') {
        // Check if the string contains a hyphen
        return !value.includes('-')
      }
      // Return true for non-string values (to avoid validation)
      return true
    }),
})

export const currencyIdSchemaFn = (assets: string[]) => {
  return string().oneOf(assets).required()
}
