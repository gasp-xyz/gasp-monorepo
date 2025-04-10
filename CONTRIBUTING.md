# Contributing to GASP Monorepo

## Repository Services

| Service | Tech |
|---------|------|
| `avs-aggregator` | Go |
| `gasp-avs` | Rust |
| `contracts` | Solidity |
| `ferry-deposit`, `ferry-withdrawal`, `stash` | Node.js |
| `sequencer`, `updater` | Rust |

Environment configs in [`gasp-xyz/gasp-gitops`](https://github.com/gasp-xyz/gasp-gitops).

## Branches

| Branch | Purpose |
|--------|---------|
| `main` | Stable releases |
| `release/next` | Pre-releases |
| `develop` | Development |
| `<service>-N.N.x` | Maintenance branches (e.g., `sequencer-0.4.x`) |

## Workflow

### Development

1. **Create Branch**

   ```bash
   git checkout develop
   git checkout -b feat/feature-name-GASP-XXX  # or fix/bug-fix-GASP-XXX
   ```

2. **Make Changes**
   - Follow coding standards
   - Add tests
   - Update docs

3. **Create PR to `develop`**
   - Use [Conventional Commits](https://www.conventionalcommits.org/)
   - Get code review

### Pre-Release

**To get a pre-release:**

1. Open PR from `develop` to `release/next`
2. Merge the PR
3. CI/CD creates pre-release tags (e.g., `sequencer-v0.2.0-rc.1`)

### Release

1. Merge `release/next` to `main`
2. CI/CD creates stable release tags
3. **Important:** Manually merge `main` back to `develop` and `release/next`

### Hotfix

1. Create branch from old version tag

   ```bash
   git checkout -b sequencer-0.4.x sequencer-v0.4.0
   ```

2. Fix and commit

   ```bash
   git commit -am "fix: Address critical bug"
   ```

3. Push to trigger release

   ```bash
   git push origin sequencer-0.4.x
   ```

Our release process uses [semantic-release](https://semantic-release.gitbook.io/semantic-release/) and [semantic-release-monorepo](https://github.com/pmowrer/semantic-release-monorepo) to handle versioning automatically.
