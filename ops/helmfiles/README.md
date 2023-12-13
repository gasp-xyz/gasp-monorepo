# Using helmfile.yaml

## Prerequisites

1. Install [[helmfile](https://github.com/helmfile/helmfile)]
2. Configure connection to GKE `mangata-dev-alpha` cluster

```bash
gcloud container clusters get-credentials mangata-dev-alpha --region europe-west1
```

## How to deploy to goerli

```bash
# Set ENVIRONMENT variable to `goerli`
export ENVIRONMENT=goerli
# Set image tag to deploy
export IMAGE_TAG=latest

# Deploy to `goerli` environment
helmfile sync -e goerli
```

Aggregator will be available by this URL: <https://eigen-aggregator-goerli.mangata.online/>
You can find all of the deployed resources in `eigen-goerli` namespace of GKE cluster.
