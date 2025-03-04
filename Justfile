# Default recipe
default:
    @just --list

# Create symlink from gasp-node/gasp-e2e to gasp-e2e repo
setup-gasp-e2e-link:
    @if [ -L "gasp-e2e" ]; then rm gasp-e2e; echo "Removed existing symlink"; fi
    @[ -d "../gasp-e2e" ] && ln -s ../gasp-e2e gasp-e2e && echo "✅ E2E tests linked successfully" || (echo "❌ Error: ../gasp-e2e directory not found, make sure the gasp-e2e repo is cloned at the same level as this repo" && exit 1)

# Update gasp-e2e to latest branch
update-gasp-e2e-linked-repo branch="eth-rollup-develop": check-gasp-e2e
    @cd gasp-e2e && git checkout {{branch}} && git pull origin {{branch}}

# Check if gasp-e2e exists
check-gasp-e2e:
    @[ -e "gasp-e2e" ] || just setup-gasp-e2e-link

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
