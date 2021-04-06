#! /bin/bash

set -e

echo "Building for x64 Windows"
cross build --target x86_64-pc-windows-gnu --release

echo "Creating bundle output"
rm -rf ./bundle/windows/release
mkdir -p ./bundle/windows/release

echo "Copying binary"
cp ./target/x86_64-pc-windows-gnu/release/ludum-dare-48.exe ./bundle/windows/release

echo "Copying assets"
cp -r ./assets ./bundle/windows/release

echo "Packing assets"
zip -r ./bundle/windows/release-x86_64-pc-windows-gnu.zip ./bundle/windows/release