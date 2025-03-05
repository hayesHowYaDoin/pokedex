default:
  @just --list

pre-commit-all:
  pre-commit run --all-files

build *ARGS:
  cargo build {{ARGS}}

build-deb *ARGS:
  cargo deb {{ARGS}}

run *ARGS:
  cargo run {{ARGS}}

watch *ARGS:
  bacon --job run -- -- {{ ARGS }}
