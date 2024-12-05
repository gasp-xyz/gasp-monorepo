import * as yup from 'yup'

export const getDataByWalletSchema = yup.object().shape({
  wallet: yup
    .string()
    .required('wallet is required')
    .matches(/^0x/, 'wallet address must begin with 0x'),
})
