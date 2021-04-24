#! /bin/bash

set -e

echo "Building for x64 Windows"
cross build --target x86_64-pc-windows-gnu --release

echo "Creating bundle output"
rm -rf ./bundle/windows/release
rm -rf ./bundle/windows/release-x86_64-pc-windows-gnu.zip
mkdir -p ./bundle/windows/release

echo "Copying binary"
cp ./target/x86_64-pc-windows-gnu/release/one-breath.exe ./bundle/windows/release

echo "Copying assets"
cp -r ./assets ./bundle/windows/release

echo "Packing assets"
cd ./bundle/windows/release
zip -r ../release-x86_64-pc-windows-gnu.zip ./