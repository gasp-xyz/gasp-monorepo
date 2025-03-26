# Gasp Monorepo

> **Warning:** For testnet use only. Not production-ready.

## Project Overview

Gasp is a Layer 2 solution utilizing Eigen Layer for security and consensus:

| Component | Directory | Description |
|-----------|-----------|-------------|
| **Gasp ETH Contract** | `contracts/` | Handles ERC20 deposits and withdrawals on Ethereum |
| **Sequencer** | `sequencer/` | Processes transactions and submits them to the collator |
| **Collator** | `gasp-node/` | Builds L2 blocks using Substrate framework |
| **Updater** | `updater/` | Monitors Eigen Layer tasks and submits results to ETH contract |
| **Aggregator & Task Manager** | `avs-aggregator/` | Generates tasks and aggregates operator results |
| **Gasp AVS** | `gasp-avs/` | Decentralized operators that verify Merkle roots and re-execute blocks |
| **Ferry Services** | `ferry-deposit/`, `ferry-withdrawal/` | Handle cross-chain asset transfers |
| **Stash** | `stash/` | Node.js service for data management |
| **Eigen ETH Contracts** | `contracts/lib/eigenlayer-contracts/` | Manages storage Merkle roots and operator lists |

## Setup Guide

### Prerequisites

- **Foundry**: Required for running local testnet

  ```bash
  curl -L https://foundry.paradigm.xyz | bash
  foundryup
  ```

- **Rust Toolchain**: Required for Rust-based services

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Go Toolchain**: With zap-pretty for logging

  ```bash
  go install github.com/maoueh/zap-pretty@latest
  ```

- **Proto & Moon**: For toolchain and task management

  ```bash
  # Install proto for toolchain management
  bash <(curl -fsSL https://moonrepo.dev/install/proto.sh)
  
  # Run `proto install` to install toolchain dependencies and setup your machine for development
  proto install
  ```

- **Docker Compose**: For running multi-container setup
  See [Docker Compose installation instructions](https://docs.docker.com/compose/install/) for platform-specific instructions

### Development Setup

1. Clone and initialize submodules:

   ```bash
   git clone https://github.com/gasp-xyz/gasp-monorepo.git
   cd gasp-monorepo
   git submodule update --init --recursive
   ```

2. Setup Git hooks with Husky:

   ```bash
   # Proto will automatically install dependencies and set up Husky
   proto install
   ```

   Our repository uses:
   - [Husky](https://typicode.github.io/husky/) for Git hooks
   - [commitlint](https://commitlint.js.org/) for validating commit messages using the [Conventional Commits](https://www.conventionalcommits.org/) standard

3. Follow the [contribution guidelines](CONTRIBUTING.md) for branch naming and commit formats

### Proto Shell Activation (Optional)

For an optimized development experience in a complex monorepo with multiple toolchains (Rust, Go, Node.js), you can set up Proto's shell activation. This is particularly helpful as switching between services with different toolchain requirements can become cumbersome:

1. Add shell activation to your profile:

   ```bash
   # For Bash (~/.bashrc)
   eval "$(proto activate bash)"
   
   # For Zsh (~/.zshrc)
   eval "$(proto activate zsh)"
   ```

2. Restart your terminal or source your profile file

Benefits for Gasp monorepo development:

- Handles automatic switching between multiple toolchains when navigating between service directories
- Recognizes not only `.prototools` files but also `.nvmrc` files for Node.js version management
- Automatically loads and exports the correct environment variables for each service
- Ensures all developers use identical toolchain versions across the codebase
- Prevents the common "wrong version" issues when working with multiple services

The different services in our monorepo (Go-based `avs-aggregator`, Rust-based `sequencer`, Node.js-based `ferry` services) each require specific tool versions and configurations. With Proto shell activation, these are managed automatically as you navigate through the codebase.

> **Note:** Shell activation should be added *after* all PATH modifications in your shell profile, as any PATH changes made after the activation line will be lost when activation triggers. For detailed setup instructions for different shells and additional options, see the [Proto Activate Command documentation](https://moonrepo.dev/docs/proto/commands/activate#setup).

For general information on Proto workflows, see the [Proto Workflows documentation](https://moonrepo.dev/docs/proto/workflows).

### Running Locally

#### Using Docker Compose

```bash
# Build Docker images
moon :build-image-local

# Start all services (for M1/M2 Macs, disable Rosetta emulation in Docker Desktop)
docker compose up -d

# Tear down
docker compose down -v
```

## Technical Architecture

### AVS Architecture

The architecture of the AVS contains:

- [Eigenlayer core](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) contracts
- AVS contracts
  - [ServiceManager](contracts/src/MangataServiceManager.sol) - Will eventually contain slashing logic
  - [TaskManager](contracts/src/MangataTaskManager.sol) - Handles task creation and responses
  - [Registry contracts](https://github.com/Layr-Labs/eigenlayer-middleware) - Manages operator registration
- Task Generator - Creates tasks for block verification (currently part of the aggregator)
- Aggregator - Collects and verifies BLS signatures from operators
- Gasp AVS Operators - Execute blocks and provide signatures

### AVS Task Flow

1. Task Generator publishes tasks to the `createNewTask` function, specifying block numbers and quorum thresholds
2. Registered operators (with minimum 1 delegated token) execute the specified blocks
3. Operators sign results using BN254 curve and send signatures to the aggregator
4. Aggregator collects signatures and posts aggregated responses that meet threshold requirements
5. Responses within the response window enter dispute resolution (not yet implemented)

### Stake Updates

AVS Registry contracts have a stale view of operator shares in the delegation manager contract. To update the stake table, operators need to periodically call the [StakeRegistry.updateStakes()](https://github.com/Layr-Labs/eigenlayer-middleware/blob/f171a0812126bbb0bb6d44f53c622591a643e987/src/StakeRegistry.sol#L76) function. Currently, we use an internal script to handle this manually.

### Integration Testing

For detailed testing instructions, see the [integration tests README](tests/integration/README.md).

### AVS Node Specification

Gasp AVS nodes are being developed to comply with the [Eigenlayer AVS Node Specification](https://eigen.nethermind.io/), which requires:

- Implementation of the [AVS Node API](https://eigen.nethermind.io/docs/category/avs-node-api)
- Implementation of [Eigen prometheus metrics](https://eigen.nethermind.io/docs/category/metrics)

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our development workflow, branch strategy, and release process.
