# Adding a Board

Fork the repo and create a TOML file in the `recipes/` directory at the root of the repository. The filename should match the board name (e.g. `recipes/my-board.toml`).

## Structure

A recipe file defines a tree of toolchains. Fork walks the tree, runs `detect` rules at each node, accumulates `layer` lines into a Dockerfile, and returns any leaf node that has a `cmd` as a buildable recipe.

```toml
name = "my-recipe"

[<toolchain>]
detect = [...]
layer  = [...]

[<toolchain>.<build-system>.<compiler>.<target>.<framework>]
detect = [...]
layer  = [...]
cmd    = "cargo build --release"
```

The nesting depth is flexible — add as many levels as you need to disambiguate variants. Each TOML table key becomes a segment of the recipe label (e.g. `rust → cargo → rustc → thumbv7emnoneeabihf → embassy`).

## Fields

### `name`

The unique board identifier. Used with `fork build -c <name>`. Must be lowercase.

### `detect`

An array of rules that must **all** pass for this node to be selected.

```toml
detect = [
    { file = "Cargo.toml", key = "embassy-nrf" },
    { file = "rust-toolchain.toml" },
]
```

| Field  | Description |
|--------|-------------|
| `file` | Glob pattern relative to the project root. The rule passes if at least one matching file exists. |
| `key`  | *(optional)* String to search for inside the matched file(s). For TOML/JSON files the key names and string values are searched; for all other files a plain substring match is used. |

### `layer`

An array of Dockerfile lines accumulated from the root down to the matched leaf. Each entry is either a literal string or a templated object.

**Literal:**
```toml
layer = [
    "FROM rust:latest",
    "RUN rustup target add thumbv7em-none-eabihf",
]
```

**Templated** — reads a value from a file in the project and substitutes `${var}`:
```toml
layer = [{ cmd = "FROM rust:${var}", var = { file = "rust-toolchain.toml", key = "channel" } }]
```

The `var` object also accepts an optional `map` table to rename values before substitution:
```toml
var = { file = "rust-toolchain.toml", key = "channel", map = { "stable" = "latest" } }
```

### `cmd`

The build command to run inside the container. A node with `cmd` is a leaf — Fork will not descend further. Nodes without `cmd` are interior nodes used only for grouping and layer accumulation.


## Example: Rust + Arduino board

```toml
name = "my-board"

[rust]
detect = [{ file = "Cargo.toml", key = "edition" }]
layer  = [{ cmd = "FROM rust:${var}", var = { file = "rust-toolchain.toml", key = "channel" } }]

[rust.cargo.rustc.thumbv7emnoneeabihf.myhal]
detect = [{ file = "Cargo.toml", key = "my-hal" }]
layer  = [
    "RUN rustup target add thumbv7em-none-eabihf",
    "RUN cargo install flip-link",
]
cmd = "cargo build --release"

[arduino]
detect = [{ file = "*.ino" }]
layer  = ["FROM arduino/arduino-cli:latest"]
cmd    = "arduino-cli core install vendor:core --additional-urls https://example.com/package_index.json && arduino-cli compile --fqbn vendor:core:myboard --output-dir build ."
```

## Detection and layer accumulation

For a project that has a `Cargo.toml` containing `my-hal`, Fork builds this Dockerfile:

```dockerfile
FROM rust:1.80        # from [rust].layer (channel read from rust-toolchain.toml)
RUN rustup target add thumbv7em-none-eabihf  # from the leaf layer
RUN cargo install flip-link
```

And runs `cargo build --release` inside the resulting container.

## Tips

- **Table key names** can be anything but must be valid TOML bare keys (alphanumeric + `_`). Choose names that reflect the target or framework clearly, as they appear in the recipe label shown to users.
- **Layer lines are raw Dockerfile instructions.** The first `FROM` encountered sets the base image; subsequent lines must be valid `RUN`, `COPY`, `ENV`, etc. instructions.
- **`detect` is optional.** A node without `detect` always matches and is always included when a child matches.
