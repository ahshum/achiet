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
    node yarn run build
    rust cargo build --release
    ;;
esac
