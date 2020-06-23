docker build . -t verus:latest
docker create -it --name dummy verus:latest bash
docker cp dummy:/verus/verus-wasm/pkg www/
docker cp dummy:/verus/verus-ruby-wrappers/target/release/libverus_ruby_wrappers.dylib verus-gem/bin/
docker rm -f dummy

cd ruby-server
bundle
cd ..

