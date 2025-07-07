# Stash

Stash is a service that serves some configurations and also caches some blockchain node information.

## API

### Hosts

| Environment                                                                              | URL |
|------------------------------------------------------------------------------------------|----|
| [FRONTEND](https://gasp-stash-frontend-dot-direct-pixel-353917.oa.r.appspot.com)    | `https://gasp-stash-frontend-dot-direct-pixel-353917.oa.r.appspot.com` |
| [HOLESKY](https://gasp-stash-holesky-dot-direct-pixel-353917.oa.r.appspot.com) | `https://gasp-stash-holesky-dot-direct-pixel-353917.oa.r.appspot.com` |
| [PROD](https://gasp-stash-prod-dot-direct-pixel-353917.oa.r.appspot.com/)    | `https://gasp-stash-prod-dot-direct-pixel-353917.oa.r.appspot.com/` |

### API documentation

To access the public API, refer to the ```[HOST]/doc``` endpoint, where you'll find comprehensive documentation for our services.
This resource provides detailed information about our available API functionalities and how to interact with them.

### Postman collections

The `api/` can be imported into Postman to use the API cals.
Setup the `$host` env to frontent & holesky, or replace it with actual URL in the API address bar.
Collection contains scripts to visualize the response data in a graph.
Script is defined in the `Tests` tab of the request section, and graph is in the `Body -> Visualize` tab in the response section.

Each API has a limit of 50K entries to return.

## Local quick start

1. Rename `.env.example` to `.env` and add missing envs (if any)
2. Install all the dependencies `yarn`
3. Start local Redis `docker run --name stash-redis-stack -p 6379:6379 -d redis/redis-stack-server`
4. Make a copy of `.env.example` and name it `.env`. This file will be ignored from git and you can set your own values.
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
3. Port forward into our cloud instance `gcloud compute ssh port-forward-vm --zone=europe-west6-a -- -N -L 6379:10.96.14.131:6379` (dev Redis)
4. You can test your connection with `redis-cli ping`
5. You should be able to connect to the Redis instance of your choice thru localhost setup: `127.0.0.1` and NO password
6. You can now modify the values of XCM network configurations.

## Configurations

### Port forwards instance

`port-forward-vm  europe-west6-a  f1-micro 10.172.0.2 34.65.196.67`

### Port forward command

`gcloud compute ssh port-forward-vm --zone=europe-west6-a -- -N -L 6379:REPLACE_WITH_REDIS_INERNAL_IP:6379`

### HOLESKY

`10.37.94.163`

### Access secrets via SOPS

```bash
# 1. Install sops from here https://github.com/getsops/sops

# 2. Ensure you local machine has gcloud CLI installed and configured:
gcloud auth login
gcloud auth application-default login

# Configure git cli to automatically show you diffs on secret changes when you have access to them
git config --global diff.sopsdiffer.textconv "sops -d --config /dev/null"

# To edit secrets.enc.yaml file run next command
sops holesky.enc.env
```

## Running with Docker Compose

To run Stash locally using Docker Compose:

1. Make sure Docker and Docker Compose are installed on your system
2. Create a `.env` file from the provided example and add missing values

   ```bash
   cp .env.example .env
   ```

3. Start the application stack:

   ```bash
   docker compose up --build -d
   ```

   This will start:
   - Stash API service
   - Redis for main cache
   - Redis TimeSeries for time-series data

4. Access the application at `http://localhost:8080`
5. Access Redis Insight UI at:
   - Main Redis: `http://localhost:8001`
   - TimeSeries Redis: `http://localhost:8002`

6. To view logs:

   ```bash
   docker compose logs -f
   ```

7. To stop the services:

   ```bash
   docker compose down
   ```

### Troubleshooting Redis Connections

If you encounter Redis authentication errors:

- Ensure REDIS_PASS and TIMESERIES_PASS in your .env or compose.yml are properly set with empty defaults:

  ```yaml
  REDIS_PASS: ${REDIS_PASS:-}
  TIMESERIES_PASS: ${TIMESERIES_PASS:-}
  ```

- For password-protected Redis, set the same password values for Redis containers and environment variables

## Moon Command Usage

[Moon](https://moonrepo.dev/) commands can be run using:

- Directory selector: `moon ~:<task>` (from current directory)

  ```bash
  moon ~:lint    # Lint code
  moon ~:format  # Format code
  moon ~:test    # Run unit tests
  ```

- Project name: `moon stash:<task>` (from anywhere)

  ```bash
  moon stash:lint               # Lint code
  moon stash:build-image-local  # Build Docker image
  ```

View all tasks: `moon project stash`
