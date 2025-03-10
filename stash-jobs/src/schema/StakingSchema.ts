import { lazy, mixed, number, object, string } from 'yup'
export const dailyRewardDateSchema = object().shape({
  date: string()
    .matches(/^\d{2}-\d{2}-\d{4}$/)
    .optional(),
})
