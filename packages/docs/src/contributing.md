# Contributing

PRs are welcome. If you're adding a new board or toolchain, test it on real hardware before submitting.

## Adding a Board

Create a TOML file in `boards/` named after the MCU (e.g. `boards/nrf52840.toml`). See [Board Definition Syntax](./toml-syntax.md) for the full reference.

Minimal example:

```toml
name = "nrf52840"

[rust]
detect = [{ file = "Cargo.toml", key = "edition" }]
layer  = [{ cmd = "FROM rust:${var}", var = { file = "rust-toolchain.toml", key = "channel" } }]

[rust.cargo.rustc.thumbv7emnoneeabihf]
detect = [{ file = ".cargo/config.toml", key = "thumbv7em-none-eabihf" }]
layer  = ["RUN rustup target add thumbv7em-none-eabihf"]
cmd    = "cargo build"
```

The board is picked up automatically at compile time — no registration needed.

## Local Development

```bash
# Build everything
cargo build

# Run the CLI against a local workspace
cargo run -p cli -- build --mcu rp2040 ./path/to/project
```

## Project Structure

```
boards/           # Board TOML definitions
packages/
├── cli/          # Clap-based CLI (commands, UI)
├── boards/       # TOML parser and toolchain tree engine
└── docs/         # This documentation
```
