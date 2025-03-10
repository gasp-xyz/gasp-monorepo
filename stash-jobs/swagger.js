import swaggerAutogen from 'swagger-autogen'
import { createRequire } from 'module'
const require = createRequire(import.meta.url)
const { version } = require('./package.json')

const doc = {
  info: {
    title: 'Mangata Stash API',
    version,
    description:
      'Mangata Stash is a service that serves some configurations and also caches some blockchain node information.',
  },
  servers: [
    {
      url: 'http://localhost:8080',
      description: 'localhost',
    },
    {
      url: 'https://mangata-stash-dev-dot-direct-pixel-353917.oa.r.appspot.com',
      description: 'Development',
    },
    {
      url: 'https://mangata-stash-prod-dot-direct-pixel-353917.oa.r.appspot.com',
      description: 'Production',
    },
  ],
}

const outputFile = './swagger-output.json'
const routes = ['./src/app']

/* NOTE: If you are using the express Router, you must pass in the 'routes' only the
root file where the route starts, such as index.js, app.js, routes.js, etc ... */

swaggerAutogen(outputFile, routes, doc)
