FROM rust:1.82-alpine
RUN apk --no-cache add build-base

RUN rustup component add rustfmt \
  && rustup target add wasm32-unknown-unknown

RUN apk --no-cache --virtual .deps add curl \
  && curl -sL https://github.com/trunk-rs/trunk/releases/download/v0.21.5/trunk-x86_64-unknown-linux-musl.tar.gz -o ./trunk.tar \
  && tar -xf ./trunk.tar \
  && chmod +x ./trunk \
  && mv ./trunk /usr/local/bin \
  && rm ./trunk.tar \
  && curl -sL https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-linux-x64 -o ./tailwindcss \
  && chmod +x ./tailwindcss \
  && mv ./tailwindcss /usr/local/bin \
  && apk del .deps
