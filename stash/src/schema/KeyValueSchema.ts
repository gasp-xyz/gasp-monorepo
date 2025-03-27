import * as yup from 'yup'

export const keyValueSchema = yup.object().shape({
  key: yup.string().required('key is required'),
  value: yup.string().required('value is required'),
})
