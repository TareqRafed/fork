# Fork

> **Beta software.** Fork is still in early development and may have rough edges. If you run into issues, please [open a PR](https://github.com/TareqRafed/fork/pulls).

A CLI tool that handles the annoying parts of multi-MCU development: detecting boards, building firmware in isolated Docker containers, and flashing devices.

**[Documentation](packages/docs/src/introduction.md)** — or build it locally with `mdbook serve packages/docs`.

## The Problem

You're working on a project that targets multiple MCUs. Each one needs different toolchains, SDKs, and build systems. Your machine is cluttered with conflicting tool versions. CI is a mess. New team members spend a day setting up their environment.

## What Fork Does

1. **Detects** connected MCUs via USB (VID/PID matching)
2. **Builds** firmware in Docker containers with the right toolchain
3. **Flashes** the resulting binary to your device

```bash
# Plug in your board, then:
fork build
fork flash
```

Fork auto-detects which build tool your project uses by checking your source files. Using `embassy-rp`? It knows. Using the Pico SDK with CMake? It knows. No configuration needed for common setups.

## Why not just use Docker directly?

You could. For a single MCU with one toolchain, a Makefile wrapping `docker run` is probably simpler. Fork pays off when:

- **Multiple MCUs** — you'd otherwise maintain separate `docker run` invocations with different images, flags, mount paths, and artifact locations per board
- **Team onboarding** — `fork build` works for everyone immediately; no one has to know the right image tag or target triple
- **Board detection** — Fork reads the USB bus and selects the right toolchain automatically; you don't specify the board, you just plug it in
- **Consistent flash step** — `fork flash` knows the artifact path and flash tool for the detected board; no copy-pasting `elf2uf2-rs` incantations

Without Fork, targeting an RP2040 with embassy and an ESP32-C3 with esp-idf in the same repo looks like:

```bash
# RP2040
docker run --rm -v $(pwd):/project \
  ghcr.io/embassy-rs/embassy:latest \
  cargo build --release --target thumbv6m-none-eabi
elf2uf2-rs target/thumbv6m-none-eabi/release/firmware firmware.uf2
# ... copy to device

# ESP32-C3
docker run --rm -v $(pwd):/project \
  espressif/idf:latest \
  idf.py build
espflash flash build/firmware.bin
```

With Fork:

```bash
fork build
fork flash
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

## Supported Boards

| Board | Build Tools | Flash Tool |
|-------|-------------|------------|
| RP2040 | pico-sdk, embassy-rp, rp2040-hal | elf2uf2-rs |
| ESP32-C3 | esp-idf, esp-hal | espflash |
| STM32F405 | embassy-stm32, stm32f4xx-hal | dfu-util |

## Adding a Board

Create a TOML file in `boards/`:

```toml
name = "your-board"
flash_tool = "your-flash-tool"

[usb]
vid = 0x1234
pid = [0x5678, 0x5679]

[[build_tools]]
name = "some-hal"
docker_image = "rust:latest"
build_command = ["cargo", "build", "--release", "--target", "thumbv7em-none-eabihf"]
artifact_path = "target/thumbv7em-none-eabihf/release/firmware.bin"
detect_command = "grep -q 'some-hal' Cargo.toml"
```

The `detect_command` runs in your project directory. If it exits 0, Fork considers that build tool compatible with your project.

## FAQ

**I've been doing embedded for 20 years. I have Makefiles. Why add another tool?**

You don't need to. If your Makefiles work and your team is comfortable, Fork offers nothing you don't already have. It's aimed at teams without that institutional knowledge — or projects that span enough boards that maintaining those Makefiles gets tedious.

**What if I need to customize the build?**

The `build_command` in the board TOML is the full command passed to Docker — you control it. For anything more dynamic, `--tool` lets you select a specific build configuration, and you can always fall back to running Docker manually.

**My board isn't supported.**

Add a TOML file in `boards/` and open a PR. The format is straightforward and documented in [Adding a Board](packages/docs/src/adding-a-board.md).

**What about WSL / Windows?**

Not tested yet. USB passthrough to WSL requires `usbipd`. Contributions welcome.

## Contributing

PRs welcome. If you're adding support for a new board or build tool, include the TOML config and test it on real hardware if possible.

## License

MIT
