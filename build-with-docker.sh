docker build . -t verus:latest
docker create -it --name dummy verus:latest bash
docker cp dummy:/verus/verus-wasm/pkg www/
docker rm -f dummy

cd www
npm i
