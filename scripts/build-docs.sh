#!/bin/bash
set -e

# Install Rust if not available
if ! command -v cargo &>/dev/null; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
  source "$HOME/.cargo/env"
fi

# Install mdBook if not available
if ! command -v mdbook &>/dev/null; then
  cargo install mdbook
fi

# Pre-build the custom preprocessor so `cargo run` in book.toml is a no-op compile
cargo build -p mdbook-boards

# Build the docs
mdbook build packages/docs
