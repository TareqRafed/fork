<h1><img src="assets/fork-logo.png" alt="Fork Logo" width="48" style="vertical-align: middle; margin-right: 8px;"/> Fork</h1>

> **Beta software.** Fork is still in early development. If you run into issues, please [open a PR](https://github.com/TareqRafed/fork/pulls).

CLI to build any firmware for any MCUs*, No abstractions for the build system or any dotfiles required.

Simply `fork build -m stm32f405`
You can also do `fork build -m stm32f405 -- --release [whatever your buildsystem takes]`

Fork, will extract the correct versions, compilers and toolchain versions based on your workspace configurations, build docker OCI and run the image.

**[Documentation](packages/docs/src/introduction.md)**

## The Problem

You're working on a project that targets multiple MCUs. Each one needs different toolchains, SDKs, and build systems. Your machine is cluttered with conflicting tool versions but you just want to get started.


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

With Fork:

```bash
fork build -m rp2040 ./firmware/rp2040 && fork build -m esp32 ./firmware/esp32
```


## Installation

```bash
cargo install --path packages/cli
```

Requires Docker.

## Usage

```bash
# Detect connected MCUs
fork detect

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
name = "your-board"

[[build_tools]]
name = "some-hal"
docker_image = "rust:latest"
build_command = ["cargo", "build", "--release", "--target", "thumbv7em-none-eabihf"]
artifact_path = "target/thumbv7em-none-eabihf/release/firmware.bin"
detect_command = "grep -q 'some-hal' Cargo.toml"
```

## Supported Boards

> You can easily add a board by opening a PR

| Board | Build Tools | Flash Tool |
|-------|-------------|------------|
| RP2040 | pico-sdk, embassy-rp, rp2040-hal | elf2uf2-rs |
| ESP32-C3 | esp-idf, esp-hal | espflash |
| STM32F405 | embassy-stm32, stm32f4xx-hal | dfu-util |

## FAQ

**I've been doing embedded for 20 years. I have Makefiles. Why add another tool?**

You don't need to. If your Makefiles work and your team is comfortable, Fork offers nothing you don't already have. It's aimed at teams without that institutional knowledge — or projects that span enough boards that maintaining those Makefiles gets tedious.

**What if I need to customize the build?**

The `build_command` in the board TOML is the full command passed to Docker — you control it. For anything more dynamic, `--tool` lets you select a specific build configuration.

**My board isn't supported.**

Add a TOML file in `boards/` and open a PR. The format is straightforward and documented in [Adding a Board](packages/docs/src/adding-a-board.md).

**What about WSL / Windows?**

Should work, Contributions welcome.

## Contributing

PRs welcome. If you're adding support for a new board or build tool, include the TOML config and test it on real hardware if possible.

## License

MIT
