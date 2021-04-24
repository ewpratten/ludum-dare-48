#! /bin/bash

set -e

# Build all platforms
./bundle/linux/create-release.sh
./bundle/windows/create-release.sh

# Make a uni-bundle
echo "Creating a fat bundle for all platforms"
rm -rf ./bundle/release
rm -rf ./bundle/one-breath.zip
mkdir -p ./bundle/release
cp -r ./assets ./bundle/release
cp ./bundle/linux/release/one-breath ./bundle/release/one-breath
cp ./bundle/windows/release/one-breath.exe ./bundle/release/one-breath.exe
cd ./bundle/release
zip -r ../one-breath.zip ./