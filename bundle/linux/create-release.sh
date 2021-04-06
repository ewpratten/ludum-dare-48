#! /bin/bash

set -e

echo "Building for x64 Linux"
docker build -t ldjam_48_x86_64_unknown_linux_gnu_build_env ./bundle/linux
cross build --target x86_64-unknown-linux-gnu --release

echo "Creating bundle output"
rm -rf ./bundle/linux/release
mkdir -p ./bundle/linux/release

echo "Copying binary"
cp ./target/x86_64-unknown-linux-gnu/release/ludum-dare-48 ./bundle/linux/release

echo "Copying assets"
cp -r ./assets ./bundle/linux/release

echo "Packing assets"
zip -r ./bundle/linux/release-x86_64-unknown-linux-gnu.zip ./bundle/linux/release