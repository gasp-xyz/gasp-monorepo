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
  type: yup
    .string()
    .required('type is required')
    .oneOf(['deposit'], 'type must be "deposit"'),
  chain: yup
    .string()
    .required('chain is required')
    .oneOf(
      ['Ethereum', 'Arbitrum', 'Base', 'Sonic'],
      'network must be either "Ethereum", "Arbitrum", "Base" or "Sonic"'
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
  type: yup
    .string()
    .required('type is required')
    .oneOf(
      ['deposit', 'withdrawal'],
      "type must be either 'deposit' or 'withdrawal'",
    ),
})

export const getAllTransactionsByAddressSchema = yup.object().shape({
  address: yup.string().required('address is required'),
  type: yup
    .string()
    .required('type is required')
    .oneOf(
      ['deposit', 'withdrawal'],
      "type must be either 'deposit' or 'withdrawal'",
    ),
})

export const getAllTransactionsByAddressAndStatusSchema = yup.object().shape({
  address: yup.string().required('address is required'),
  status: yup.string().required('status is required'),
  type: yup
    .string()
    .required('type is required')
    .oneOf(
      ['deposit', 'withdrawal'],
      "type must be either 'deposit' or 'withdrawal'",
    ),
})

export const getTransactionByEntityIdSchema = yup.object().shape({
  entityId: yup.string().required('entityId is required'),
  type: yup
    .string()
    .required('type is required')
    .oneOf(
      ['deposit', 'withdrawal'],
      "type must be either 'deposit' or 'withdrawal'",
    ),
})
