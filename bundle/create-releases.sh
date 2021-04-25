#! /bin/bash

set -e

# Build all platforms
./bundle/linux/create-release.sh
./bundle/windows/create-release.sh

# Make a uni-bundle
echo "Creating a fat bundle for all platforms"
rm -rf ./bundle/release
rm -rf ./bundle/ldgame.zip
mkdir -p ./bundle/release
cp -r ./assets ./bundle/release
cp ./bundle/linux/release/ldgame ./bundle/release/ldgame
cp ./bundle/windows/release/ldgame.exe ./bundle/release/ldgame.exe
cd ./bundle/release
zip -r ../ldgame.zip ./