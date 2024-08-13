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
