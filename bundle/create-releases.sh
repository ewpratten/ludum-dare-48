#! /bin/bash

set -e

# Build all platforms
./bundle/linux/create-release.sh
./bundle/windows/create-release.sh

# Make a uni-bundle
echo "Creating a fat bundle for all platforms"
rm -rf ./bundle/release
rm -rf ./bundle/pink-man-swim.zip
mkdir -p ./bundle/release
cp -r ./assets ./bundle/release
cp ./bundle/linux/release/pink-man-swim ./bundle/release/pink-man-swim
cp ./bundle/windows/release/pink-man-swim.exe ./bundle/release/pink-man-swim.exe
cd ./bundle/release
zip -r ../pink-man-swim.zip ./