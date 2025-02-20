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
