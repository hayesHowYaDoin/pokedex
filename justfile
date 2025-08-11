default:
  @just --list

pre-commit-all:
  pre-commit run --all-files

build *ARGS:
  cargo build {{ARGS}}

build-deb *ARGS:
  cargo deb {{ARGS}}

run *ARGS:
  ./target/debug/rich_pokedex {{ARGS}}

watch *ARGS:
  bacon --job run -- -- {{ ARGS }}
