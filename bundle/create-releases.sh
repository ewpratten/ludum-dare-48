#! /bin/bash

set -e

# Build all platforms
./bundle/linux/create-release.sh
./bundle/windows/create-release.sh