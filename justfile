default:
  @just --list

pre-commit-all:
  pre-commit run --all-files

build *ARGS:
  cargo build {{ARGS}}

build-deb *ARGS:
  cargo deb {{ARGS}}

lint:
  cargo clippy --workspace --all-targets -- -D warnings
  cargo fmt --all -- --check

test:
  cargo test --workspace

run *ARGS:
  ./target/debug/rich_pokedex {{ARGS}}

watch *ARGS:
  bacon --job run -- -- {{ ARGS }}
