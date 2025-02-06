default:
  @just --list

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
  pre-commit run --all-files

# Run 'cargo build' on the project
build *ARGS:
  cargo build {{ARGS}}

# Run 'cargo run' on the project
run *ARGS:
  cargo run {{ARGS}}

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
  bacon --job run -- -- {{ ARGS }}

# Build the .deb package
build-deb:
  cargo deb
