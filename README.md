<h1><img src="assets/fork-logo.png" alt="Fork Logo" width="48" align="center" style="vertical-align: middle; margin-right: 8px;"/> Fork</h1>

> **Beta software.** Fork is still in early development. If you run into issues, please [open a PR](https://github.com/TareqRafed/fork/pulls).

Fork scans your project and resolves your workspace into a ready container to build firmware without manual configuration. It doesn't require dotfiles or enforce build constraints, making it easy to hook into any project.

> Currently only handful MCUs are defined, but could be easily added by opening a PR and add a recipe definition into `/recipes` 

Example:

![Example output](assets/example.svg)

You can also do:

```zsh
# Build firmware
fork build -c stm32f405 -- [build command args]
fork build -c stm32f405 -- --release  # uses cargo build --release 

# Example: run custom commands inside container 
fork run -c stm32f405 -- "[command your build system uses]"
fork run -c stm32f405 -- "cargo test"
fork run -c stm32f405 -- "ls -la"
```

Fork detects your project's toolchain from config files, builds a Dockerfile and runs your build inside that container. No config files required in your project beyond what your build system already has.

This was possible with a lower maintenance burden because it enables Docker-style builds to branch based on workspace files. This model maps naturally to embedded workflows—unlike standard software development, which has fewer build variables (no platform).

**[Documentation](packages/docs/src/introduction.md)**

## Why not just use Docker directly?

Without Fork, targeting an RP2040 with embassy and an ESP32-C3 with esp-idf in the same repo looks like:

```bash
# Build RP2040 - Hope you remembered the UID mapping and target path!
docker run --rm -u $(id -u):$(id -g) -v $(pwd):/project -w /project \
  ghcr.io/embassy-rs/embassy:latest \
  sh -c "cargo build --release && cp target/thumbv6m-none-eabi/release/app.uf2 ."

# Build ESP32 - Different image, different CLI, different output folder
docker run --rm -v $(pwd):/project -w /project \
  espressif/idf:v5.1 \
  sh -c "idf.py build && cp build/app.bin ."
```

Fork allows you to do:

```bash
fork build -c rp2040 ./firmware/rp2040 && fork build -c esp32 ./firmware/esp32
```


## Install

Requires Docker or Podman.

```bash
TODO
```


## Adding a Recipe

Open a PR, create a TOML file in `recipes/`:

```toml
name = "example-recipe"

[language]
# Detect if this project uses this language
detect = [{ file = "project.config", key = "language_enabled" }]

[language.runtimeA]
# If project.config contains runtimeA = true where TOML and JSON behave as key-value but other formats as find
detect = [{ file = "project.config", key = "runtimeA" }]
layer = [
  "RUN install target-platform",
  "RUN install flashing-tool"
]
cmd = "build project"


[language.runtimeB]
# If project.config contains runtimeB = true
detect = [{ file = "project.config", key = "runtimeB" }]


[language.runtimeB.variant1]
# Further specialization
detect = [{ file = "project.config", key = "variant1" }]
layer = [
  "RUN install flashing-tool"
]


[language.runtimeB.variant2]
detect = [{ file = "project.config", key = "variant2" }]
layer = [
  "RUN install alternative-tool"
]
cmd = "default build command"
```


## FAQ

**My setup isn't supported.**

Add a TOML file in `recipes/` and open a PR. 

**What about WSL / Windows?**

Should work, contributions welcome.

## Contributing

PRs welcome. If you're adding support for a build tool, include the TOML config and test it on real hardware if possible.

## License

MIT
