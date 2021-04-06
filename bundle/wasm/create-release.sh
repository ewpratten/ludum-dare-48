#! /bin/bash

set -e

echo "Building for 32-bit emscripten"
# docker build -t ldjam_48_wasm32_unknown_emscripten_build_env ./bundle/wasm
cross build --target wasm32-unknown-emscripten --release --verbose

echo "Creating bundle output"
rm -rf ./bundle/wasm/release
mkdir -p ./bundle/wasm/release

echo "Copying binary"
cp ./target/wasm32-unknown-emscripten/release/ludum-dare-48.exe ./bundle/wasm/release

echo "Copying assets"
cp -r ./assets ./bundle/wasm/release

echo "Packing assets"
zip -r ./bundle/wasm/release-wasm32-unknown-emscripten.zip ./bundle/wasm/release