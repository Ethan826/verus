# docker build . -t verus:latest
docker create -it --name dummy verus:latest bash
docker cp dummy:/verus/verus-wasm/pkg www/
docker cp dummy:/verus/verus-validations/target/release/libverus_validations.dylib verus-gem/bin/
docker rm -f dummy