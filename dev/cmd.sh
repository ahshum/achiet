#!/usr/bin/env sh

HERE="$(dirname $(readlink -f "$0"))"
COMPOSE_FILE="$HERE/docker-compose.yml"
SQLITE_FILE="app.db"
CMD_COMPOSE="docker compose -f $COMPOSE_FILE"

run_service() {
  $CMD_COMPOSE \
    run \
    --rm -it \
    -u "$UID" \
    "$@"
}

build_service() {
  $CMD_COMPOSE \
    build \
    --pull \
    "$@"
}

rust() {
  if [ $# -eq 0 ]; then
    set -- -p 8000:8000 rust sh
  else
    set -- rust "$@"
  fi

  run_service -e "HOME=/tmp" "$@"
}

rustfmt() {
  run_service rust find src/ -name '*.rs' -exec rustfmt --edition 2021 {} \;
}

web() {
  run_service \
    -p "3000:3000" \
    -p "3001:3001" \
    web sh
}

atlas() {
  run_service migrate "$@"
}

atlasfmt() {
  atlas schema fmt
}

sqlite() {
  dbfile="sqlite://${1:-$SQLITE_FILE}"
  atlas schema apply \
    --url "$dbfile" \
    --to "file://schema.hcl" \
    --var tenant=main
}

[ $# -gt 0 ] && cmd="$1" && shift
cd "$(dirname "$HERE")"
case "$cmd" in
  rust)
    rust "$@"
    ;;

  rustfmt)
    rustfmt
    ;;

  web)
    web "$@"
    ;;

  sqlite)
    sqlite "$@"
    ;;

  schemafmt|atlasfmt)
    atlasfmt
    ;;

  build)
    build_service "$@"
    ;;

  *)
    cat <<EOF
[cmd]
  rust [command] [arguments]
  rustfmt
  web
  sqlite [file]
  atlasfmt
EOF
    ;;
esac
