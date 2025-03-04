# Default recipe
default:
    @just --list

# Create a new Ubuntu development machine in Orbstack with pre-installed packages
create-dev-machine:
    orb create ubuntu dev-ubuntu -c ops/local-env/orbstack-cloudinit.yaml

# Reset development machine by removing and recreating it
reset-dev-machine:
    orb delete dev-ubuntu
    @just create-dev-machine

# Setup k8s-builder with the given number of replicas to use for building images as in CI
setup-k8s-builder replicas="1":
  @just remove-k8s-builder

  docker buildx create --name k8s-builder --driver kubernetes --driver-opt namespace=buildkit \
    --driver-opt nodeselector="github-actions-pool-arm=true" \
    '--driver-opt="tolerations=key=github-actions-pool-arm,value=true;key=CriticalAddonsOnly,value=true;key=kubernetes.io/arch,value=arm64"' \
    --driver-opt replicas={{replicas}},requests.cpu=7,requests.memory=12Gi,limits.cpu=7,limits.memory=12Gi \
    --platform linux/arm64 --bootstrap
    
  docker buildx create --append --name k8s-builder --driver kubernetes --driver-opt namespace=buildkit \
    --driver-opt nodeselector="github-actions-pool=true" \
    '--driver-opt="tolerations=key=github-actions-pool,value=true;key=CriticalAddonsOnly,value=true"' \
    --driver-opt replicas={{replicas}},requests.cpu=7,requests.memory=12Gi,limits.cpu=7,limits.memory=12Gi \
    --platform linux/amd64 --bootstrap

# Remove k8s-builder if it exists
remove-k8s-builder:
    #!/usr/bin/env bash
    set -euo pipefail
    if docker builder ls | grep k8s-builder; then
        echo "Removing existing k8s-builder..."
        docker builder rm k8s-builder
    else
        echo "No k8s-builder found, nothing to remove."
    fi
