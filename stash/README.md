# Stash

Stash is a service that serves some configurations and also caches some blockchain node information.

## API


### Hosts
| Environment                                                                           | URL                                                                   |
|---------------------------------------------------------------------------------------|-----------------------------------------------------------------------|
| [ROLLUP-DEV](https://mangata-stash-rollup-dev-dot-direct-pixel-353917.oa.r.appspot.com)     | `https://mangata-stash-rollup-dev-dot-direct-pixel-353917.oa.r.appspot.com` |
| [ROLLUP-TESTNET](https://mangata-stash-rollup-testnet-dot-direct-pixel-353917.oa.r.appspot.com) | `https://mangata-stash-rollup-testnet-dot-direct-pixel-353917.oa.r.appspot.com`  |

### API documentation
To access the public API, refer to the ```[HOST]/doc``` endpoint, where you'll find comprehensive documentation for our services.
This resource provides detailed information about our available API functionalities and how to interact with them.


### Postman collections
The `api/stash.postman-collection.json` can be imported into Postman to use the API cals.
Setup the `$host` env to rollup-dev & rollup-testnet, or replace it with actual URL in the API address bar.
Collection contains scripts to visualize the response data in a graph.
Script is defined in the `Tests` tab of the request section, and graph is in the `Body -> Visualize` tab in the response section.

Each API has a limit of 50K entries to return.

## Local quick start

1. Rename `.env.local` to `.env` and add missing envs (if any)
2. Install all the dependencies `yarn`
3. Start local Redis `docker run --name stash-redis-stack -p 6379:6379 -d redis/redis-stack-server`
4. Make a copy of `.env.local` and name it `.env`. This file will be ignored from git and you can set your own values.
5. Run the app in debug mode with watch option `yarn start-local`

## Documentation

### Used tech

- `Node.js` & `Typescript`
- `Express` - Node.js web framework.

### Tests

Tests have their own runtime docker container. Currently, most of the use cases are covered using integration tests in `integration.test.ts`

We use a combination of `supertests` & `testcontainers` libraries to achieve smooth and unified integration tests running either on local or CI environment.

## Storage
Google Memorystore (Redis with RDB)

## XCM Metadata Network Service
Data in this service are managed by

### Modification of XCM values

1. Setup `gcloud` CLI tool (make sure you setup correct project ID - `gcloud config set project direct-pixel-353917` )
2. Install Redis client that suits you
3. Port forward into our cloud instance `gcloud compute ssh port-forward-vm --zone=europe-west6-a -- -N -L 6379:10.15.245.67:6379` (rollup-dev Redis)
4. You can test your connection with `redis-cli ping`
5. You should be able to connect to the Redis instance of your choice thru localhost setup: `127.0.0.1` and NO password
6. You can now modify the values of XCM network configurations.

## Configurations

### Port forwards instance

`port-forward-vm  europe-west6-a  f1-micro 10.172.0.2 34.65.196.67`  <!-- This is not a correct addres -->

### Port forward command

`gcloud compute ssh port-forward-vm --zone=europe-west6-a -- -N -L 6379:REPLACE_WITH_REDIS_INERNAL_IP:6379`

### ROLLUP-DEV

`10.96.14.131` <!-- This is not a correct addres -->

### ROLLUP-TESTNET

`10.157.152.4` <!-- This is not a correct addres -->
