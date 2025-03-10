import 'gasp-types'
import express, { Request, Response, json, urlencoded } from 'express'
import cors from 'cors'

import { createRequire } from 'module'
const require = createRequire(import.meta.url)
const swaggerFile = require('../swagger-output.json')

const app = express()

// App configuration
app.set('port', process.env.PORT || 8080)
app.use(json())
app.use(urlencoded({ extended: true }))
app.use(cors())

//Root (Health Check)
app.get('/', async (req: Request, res: Response): Promise<void> => {
  res.json({})
})
export default app
