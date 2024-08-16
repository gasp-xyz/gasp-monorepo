import * as yup from 'yup'

export const startTracingSchema = yup.object().shape({
  txHash: yup
    .string()
    .required('txHash is required')
    .matches(/^0x/, 'txHash must begin with 0x'),
  address: yup
    .string()
    .required('address is required')
    .matches(/^0x/, 'address must begin with 0x'),
  type: yup.string().required('type is required'),
  network: yup
    .string()
    .required('network is required')
    .oneOf(
      ['ethereum', 'arbitrum'],
      'network must be either "ethereum" or "arbitrum"'
    ),
  amount: yup.string().required('amount is required'),
  asset_chainId: yup
    .string()
    .required('asset_chainId is required')
    .matches(/^0x/, 'txHash must begin with 0x'),
  asset_address: yup
    .string()
    .required('asset_address is required')
    .matches(/^0x/, 'txHash must begin with 0x'),
})

export const getStatusByTxHashOrEntityIdSchema = yup.object().shape({
  txHashOrEntityId: yup.string().required('txHash or entityId is required'),
})

export const getAllTransactionsByAddressSchema = yup.object().shape({
  address: yup.string().required('address is required'),
})

export const getAllTransactionsByAddressAndStatusSchema = yup.object().shape({
  address: yup.string().required('address is required'),
  status: yup.string().required('status is required'),
})

export const getTransactionByEntityIdSchema = yup.object().shape({
  entityId: yup.string().required('entityId is required'),
})

export const getTransactionByTxHashSchema = yup.object().shape({
  txHash: yup.string().required('txHash is required'),
})
