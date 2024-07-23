# Using helmfile.yaml

## Prerequisites

1. Install [[helmfile](https://github.com/helmfile/helmfile)]
2. Configure connection to GKE `mangata-dev-alpha` cluster
3. Install and configure SOPS to access secrets

```bash
# 1. Install sops from here https://github.com/getsops/sops

# 2. Ensure you local machine has gcloud CLI installed and configured:
gcloud auth login
gcloud auth application-default login

# Configure git cli to automatically show you diffs on secret changes when you have access to them
git config --global diff.sopsdiffer.textconv "sops -d --config /dev/null"

# To edit secrets.enc.yaml file run next command
sops secrets.enc.yaml
```

4. Configure connectin to cluster

```bash
gcloud container clusters get-credentials mangata-dev-alpha --region europe-west1
```

## How to deploy to rollup-holesky

```bash
# Set image tag to deploy
export IMAGE_TAG=latest

# Deploy to `rollup-holesky` environment
helmfile sync -e holesky
```

Aggregator will be available by this URL: <https://rollup-aggregator-holesky.gasp.xyz/>
You can find all of the deployed resources in `rollup-holesky` namespace of GKE cluster.
