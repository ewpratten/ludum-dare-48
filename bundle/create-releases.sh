#! /bin/bash

set -e

# Build all platforms
./bundle/linux/create-release.sh
./bundle/windows/create-release.sh

# Make a uni-bundle
echo "Creating a fat bundle for all platforms"
rm -rf ./bundle/release
rm -rf ./bundle/ludum-dare-48.zip
mkdir -p ./bundle/release
cp -r ./assets ./bundle/release
cp ./bundle/linux/release/ludum-dare-48 ./bundle/release/ludum-dare-48
cp ./bundle/windows/release/ludum-dare-48.exe ./bundle/release/ludum-dare-48.exe
cd ./bundle/release
zip -r ../ludum-dare-48.zip ./