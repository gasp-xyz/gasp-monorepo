import * as yup from 'yup'

export const mgxAirdropEligibilitySchema = yup.object().shape({
  address: yup.string().required('address is required'),
})

export const mgxAirdropLinkAddressSchema = yup.object().shape({
  mangataXAddress: yup.string().required('mangataXAddress is required'),
  ethereumAddress: yup
    .string()
    .required('ethereumAddress is required')
    .matches(/^0x/, 'ethereumAddress must begin with 0x'),
})

export const mgxAirdropSignatureSchema = yup.object().shape({
  signature: yup.string().required('Signature is required'),
})
