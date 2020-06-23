#!/usr/bin/env bash

# Build everything
cd verus-validations
cargo build --release
cd ../verus-ruby-wrappers
cargo build --release
cd ../verus-wasm
wasm-pack build
cd ..

# Set up Ruby
cp verus-validations/target/release/libverus_validations.dylib verus-gem/bin/
cd verus-gem
bundle
cd ../ruby-server
bundle
cd ..

# Set up Node
cd www
npm i
cd ..