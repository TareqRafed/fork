<h1><img src="assets/fork-logo.png" alt="Fork Logo" width="48" align="center" style="vertical-align: middle; margin-right: 8px;"/> Fork</h1>

> **Beta software.** Fork is still in early development. If you run into issues, please [open a PR](https://github.com/TareqRafed/fork/pulls).

CLI to build any firmware for any MCUs, without buildtool abstractions or any additional configurations. 

> Currently only handful MCUs are defined, but adding them is pretty easy, just open a PR and add a board to `/boards` 

Simply:

```zsh
fork build -m stm32f405
```

You can also do:

```zsh
fork build -m stm32f405 -- --release [whatever your buildsystem takes]
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
fork build -m rp2040 ./firmware/rp2040 && fork build -m esp32 ./firmware/esp32
```


## Install

Requires Docker or Podman.

```bash
TODO
```

## Usage

```bash
# Build for detected MCU (auto-detects build tool)
fork build

# Build with explicit MCU and build tool
fork build --mcu rp2040 --tool embassy-rp

# Flash firmware
fork flash

# Flash specific file
fork flash --file ./my-firmware.uf2
```



## Adding a Board

Open a PR, create a TOML file in `boards/`:

```toml
name = "example-board"

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

**What if I need to customize the build?**

The `build_command` in the board TOML is the full command passed to Docker — you control it.

**My board isn't supported.**

Add a TOML file in `boards/` and open a PR. The format is straightforward and documented in [Adding a Board](packages/docs/src/adding-a-board.md).

**What about WSL / Windows?**

Should work, contributions welcome.

## Contributing

PRs welcome. If you're adding support for a new board or build tool, include the TOML config and test it on real hardware if possible.

## License

MIT
