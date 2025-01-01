FROM rust:1.82-alpine
RUN apk --no-cache add build-base

RUN rustup component add rustfmt \
  && rustup target add wasm32-unknown-unknown

RUN apk --no-cache --virtual .deps add curl \
  && cd /tmp \
  && curl -sL https://github.com/trunk-rs/trunk/releases/download/v0.21.5/trunk-x86_64-unknown-linux-musl.tar.gz -o ./trunk.tar \
  && tar -xf ./trunk.tar \
  && chmod +x ./trunk \
  && mv ./trunk /usr/local/bin \
  && curl -sL https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-linux-x64 -o ./tailwindcss \
  && chmod +x ./tailwindcss \
  && mv ./tailwindcss /usr/local/bin \
  && curl -sL https://github.com/WebAssembly/binaryen/releases/download/version_121/binaryen-version_121-x86_64-linux.tar.gz -o ./binaryen.tar \
  && tar -xf ./binaryen.tar --strip-components=1 \
  && mv ./bin/* /usr/local/bin \
  && rm -rf ./* \
  && apk del .deps
