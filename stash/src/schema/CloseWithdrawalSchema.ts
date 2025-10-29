import * as yup from 'yup'

export const closeWithdrawalSchema = yup.object().shape({
  txHash: yup
    .string()
    .required('txHash is required')
    .matches(/^0x/, 'txHash must begin with 0x'),
})