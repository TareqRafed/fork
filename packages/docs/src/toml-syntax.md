# Recipe Definition

> This isn't exposed to you but it's the repo, read this if you are trying to open a PR to add more support

Board definitions are TOML files that describe how to build firmware. They are structured as a tree — each node might add Dockerfile layers and always narrows down the toolchain. Fork walks the tree, runs `detect` rules against your workspace, and follows the matching path down to a leaf. The leaf provides the final image and build command.

## Structure

```toml
name = "board-name"

[toolchain]
detect = [...]
layer  = [...]

[toolchain.sub.path]
detect = [...]
layer  = [...]
cmd    = "build command"
```

`name` is the board identifier used with `--recipe`. Everything else is a toolchain tree node.

## Node Fields

### `detect`

An array of rules checked against the user's workspace. All rules in the array must pass for the node to be selected.

```toml
detect = [
  { file = "Cargo.toml", key = "edition" },
  { file = ".cargo/config.toml", key = "thumbv6m-none-eabi" }
]
```

| Field  | Required | Description |
|--------|----------|-------------|
| `file` | yes      | Glob path relative to the workspace root |
| `key`  | no       | If provided, the file must contain this key. Works with TOML, JSON (as key-value look-up), and plain text (regex) |

If `key` is omitted, Fork only checks that the file exists.

### `layer`

An array of Dockerfile lines to append at this node. Can be literal strings or templated commands.

**Literal:**
```toml
layer = [
  "RUN rustup target add thumbv6m-none-eabi",
  "RUN cargo install flip-link"
]
```

**Templated** — reads a value from a workspace file and interpolates it:
```toml
layer = [{ cmd = "FROM rust:${var}", var = { file = "rust-toolchain.toml", key = "channel" } }]
```

The `var` rule reads `key` from `file`. If the value is not found, it falls back to `"latest"`. You can also remap values with `map`:

```toml
var = { file = "rust-toolchain.toml", key = "channel", map = { "stable" = "1.76", "nightly" = "nightly-2024-01-01" } }
```

### `cmd`

The command Fork runs inside the container. Only leaf nodes (nodes with no children) should have `cmd`.

```toml
cmd = "cargo build --release"
```

## Example

The rp2040 Rust path:

```toml
name = "rp2040"

[rust]
detect = [{ file = "Cargo.toml", key = "edition" }]
layer  = [{ cmd = "FROM rust:${var}", var = { file = "rust-toolchain.toml", key = "channel" } }]

[rust.cargo.rustc.thumbv6mnonenabi]
detect = [{ file = ".cargo/config.toml", key = "thumbv6m-none-eabi" }]
layer  = ["RUN rustup target add thumbv6m-none-eabi", "RUN cargo install flip-link"]
cmd    = "cargo build"
```

For a workspace that has `Cargo.toml` with an `edition` key and `.cargo/config.toml` with `thumbv6m-none-eabi`, Fork resolves the path `rust → cargo → rustc → thumbv6mnonenabi` and generates:

```dockerfile
FROM rust:1.76
RUN rustup target add thumbv6m-none-eabi
RUN cargo install flip-link
```
