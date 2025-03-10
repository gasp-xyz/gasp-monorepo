import { object, string } from 'yup'

export const pathFaucetSchema = object({
  toAddress: string().required().matches(/^0x/, 'toAddress must start with 0x'),
  captcha: string().required(),
})
