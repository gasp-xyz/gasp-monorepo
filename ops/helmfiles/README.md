# Using helmfile.yaml

## Prerequisites

1. Install [[helmfile](https://github.com/helmfile/helmfile)]
2. Configure connection to GKE `mangata-dev-alpha` cluster

```bash
gcloud container clusters get-credentials mangata-dev-alpha --region europe-west1
```

## How to deploy to testnet

```bash
# Set ENVIRONMENT variable to `testnet`
export ENVIRONMENT=testnet
# Set image tag to deploy
export IMAGE_TAG=latest

# Deploy to `testnet` environment
helmfile sync -e testnet
```

Aggregator will be available by this URL: <https://rollup-aggregator-testnet.mangata.online/>
You can find all of the deployed resources in `rollup-testnet` namespace of GKE cluster.
