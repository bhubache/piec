[private]
default:
  @just --list --unsorted --justfile {{justfile()}} --list-prefix ···· --list-heading ''

fmt:
  cargo fmt --version
  cargo fmt --all
  cargo clippy --version
  cargo clippy --fix --all --workspace --all-targets --all-features --locked --allow-dirty -- -D warnings

lint:
  cargo fmt --version
  cargo fmt --all -- --check
  cargo clippy --version
  cargo clippy --workspace --all-targets --all-features --locked --all -- -D warnings
