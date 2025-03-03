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

append-k8s-builder-platform-arm64:
    docker buildx create --append --name k8s-builder --driver kubernetes --driver-opt namespace=buildkit \
        --driver-opt nodeselector="github-actions-pool-arm=true" \
        '--driver-opt="tolerations=key=github-actions-pool-arm,value=true;key=CriticalAddonsOnly,value=true;key=kubernetes.io/arch,value=arm64"' \
        --driver-opt replicas=1,requests.cpu=7,requests.memory=12Gi,limits.cpu=7,limits.memory=12Gi \
        --platform linux/arm64 --bootstrap

append-k8s-builder-platform-amd64:
    docker buildx create --append --name k8s-builder --driver kubernetes --driver-opt namespace=buildkit \
        --driver-opt nodeselector="github-actions-pool=true" \
        '--driver-opt="tolerations=key=github-actions-pool,value=true;key=CriticalAddonsOnly,value=true"' \
        --driver-opt replicas=1,requests.cpu=7,requests.memory=12Gi,limits.cpu=7,limits.memory=12Gi \
        --platform linux/amd64 --bootstrap
