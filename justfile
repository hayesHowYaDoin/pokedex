default:
  @just --list

pre-commit-all:
  pre-commit run --all-files

build *ARGS:
  cargo build {{ARGS}}

package *ARGS:
  nix build .#rich_pokedex {{ARGS}}

run *ARGS:
  nix run .#rich_pokedex {{ARGS}}

watch *ARGS:
  bacon --job run -- -- {{ ARGS }}

build-deb:
  cargo deb
