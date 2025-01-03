#!/usr/bin/env sh

COMPOSE="docker compose -f ./docker-compose.dev.yml"

rust() {
  $COMPOSE run --rm -it \
    rust "$@"
}

node() {
  $COMPOSE run --rm -it \
    node "$@"
}

atlas() {
  $COMPOSE run --rm -it \
    atlas "$@"
}

cmd="$1" && [ "$#" -ge "1" ] && shift
case "$cmd" in
  init)
    mkdir -p .data
    cd .data
    echo "*" > .gitignore
    mkdir -p cargo home
    ;;

  rust)
    rust "$@"
    ;;

  node)
    node "$@"
    ;;

  atlas-fmt)
    atlas schema fmt
    ;;

  atlas-sqlite)
    dbfile="sqlite://${1:-app.db}"
    atlas schema apply \
      --url "$dbfile" \
      --to "file://schema.hcl" \
      --var tenant=main
    ;;

  b|build)
    node sh -c "pnpm install && pnpm run build"
    rust cargo build --release
    ;;

  build-wasm)
    $COMPOSE run --rm -it \
      -w /app/crates/wasm \
      node \
      pnpm install

    $COMPOSE run --rm -it \
      -e "TRUNK_BUILD_RELEASE=true" \
      -e "TRUNK_BUILD_MINIFY=true" \
      wasm \
      trunk build

    $COMPOSE run --rm -it \
      wasm \
      sh -c "\
        for f in ./dist/*.wasm; do \
          echo wasm-opt \$f; \
          mv \$f \$f.orig; \
          wasm-opt -Oz -o \$f \$f.orig; \
          rm \$f.orig; \
        done \
        "
    ;;
esac
