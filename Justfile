# Default recipe
default:
    @just --list

# Create a new Ubuntu development machine in Orbstack with pre-installed packages
create-dev-machine:
    #!/usr/bin/env bash
    orb create ubuntu \
    --name dev-ubuntu \
    --cloud-init='#cloud-config
    package_update: true
    package_upgrade: true
    packages:
      - git
      - protobuf-compiler
      - build-essential'

# Reset development machine by removing and recreating it
reset-dev-machine:
    #!/usr/bin/env bash
    orb rm -f dev-ubuntu || true
    @just create-dev-machine
