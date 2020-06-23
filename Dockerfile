# Build Rust stuff

FROM rust:1.44.1 as builder

WORKDIR /verus

COPY verus-validations verus-validations
RUN cd verus-validations && cargo build --release

COPY verus-ruby-wrappers verus-ruby-wrappers
RUN cd verus-ruby-wrappers && cargo build --release

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
COPY verus-wasm verus-wasm
RUN cd verus-wasm && wasm-pack build

# Get Ruby going

FROM ruby:2.7.1

EXPOSE 4567

WORKDIR /verus

COPY ruby-server ruby-server
COPY verus-gem verus-gem

COPY --from=builder verus/verus-ruby-wrappers/target/release/libverus_ruby_wrappers.so verus-gem/bin/
WORKDIR /verus/ruby-server
RUN gem install bundler
RUN bundle

CMD ["bundle", "exec", "ruby", "server.rb"]