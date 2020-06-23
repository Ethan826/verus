FROM rust:1.44.1

WORKDIR /verus

COPY verus-validations verus-validations
RUN cd verus-validations && cargo build --release

COPY verus-ruby-wrappers verus-ruby-wrappers
RUN cd verus-ruby-wrappers && cargo build --release

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
COPY verus-wasm verus-wasm
RUN cd verus-wasm && wasm-pack build