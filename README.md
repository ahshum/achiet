# Achiet

A bookmark manager for tracking and organizing your favourite.

## Dependencies

- docker
- docker-compose

## Build

```
./cmd.sh b
```

## Database setup

This will initialize the sqlite database (default to `app.db`) with Atlas

```
./cmd.sh atlas-sqlite
```

## Start

Parameters are environment-based.

```sh
ACHIET_DB_URL=sqlite:./app.db \
ACHIET_ADDR=0.0.0.0:3399 \
ACHIET_JWT_SECRET=secret \
RUST_LOG=achiet=debug,main=debug \
  ./target/release/main
```
