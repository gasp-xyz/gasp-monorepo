# Default recipe
default:
    @just --list

# Setup k8s-builder with the given number of replicas to use for building images as in CI
setup-k8s-builder replicas="1": remove-k8s-builder
  @echo "Creating k8s-builder with {{replicas}} replicas..."
  @docker buildx create --name k8s-builder --driver kubernetes --driver-opt namespace=buildkit \
    --driver-opt nodeselector="github-actions-pool-arm=true" \
    '--driver-opt="tolerations=key=github-actions-pool-arm,value=true;key=CriticalAddonsOnly,value=true;key=kubernetes.io/arch,value=arm64"' \
    --driver-opt replicas={{replicas}},requests.cpu=14,requests.memory=14Gi,limits.cpu=14,limits.memory=14Gi \
    --platform linux/arm64 --bootstrap
  
  @docker buildx create --append --name k8s-builder --driver kubernetes --driver-opt namespace=buildkit \
    --driver-opt nodeselector="github-actions-pool=true" \
    '--driver-opt="tolerations=key=github-actions-pool,value=true;key=CriticalAddonsOnly,value=true"' \
    --driver-opt replicas={{replicas}},requests.cpu=14,requests.memory=14Gi,limits.cpu=14,limits.memory=14Gi \
    --platform linux/amd64 --bootstrap
  @echo "✅ k8s-builder setup complete"

# Remove k8s-builder if it exists
remove-k8s-builder:
  @if docker builder ls | grep -q k8s-builder; then docker builder rm k8s-builder; fi
