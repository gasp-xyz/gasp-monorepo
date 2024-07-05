# stash service

## 1. Prerequisites

1. Install and configure SOPS to access secrets

```bash
# 1. Install sops from here https://github.com/getsops/sops

# 2. Ensure you local machine has gcloud CLI installed and configured:
gcloud auth login
gcloud auth application-default login

# Configure git cli to automatically show you diffs on secret changes when you have access to them
git config --global diff.sopsdiffer.textconv "sops -d --config /dev/null"

# To edit secrets.enc.yaml file run next command
sops rollup-dev.enc.env
```
