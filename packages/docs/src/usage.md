# Usage

## Commands

### detect

Scan for connected MCUs over USB:

```bash
fork detect
```

Fork matches connected devices by USB VID/PID against its board database and prints what it finds.

### build

Build firmware for the detected MCU:

```bash
# Auto-detect board and build tool
fork build

# Specify MCU explicitly
fork build --mcu rp2040

# Specify MCU and build tool
fork build --mcu rp2040 --tool embassy-rp
```

Fork runs the build inside a Docker container with the appropriate toolchain. The `--tool` flag overrides auto-detection if you need a specific build system.

### flash

Flash firmware to the connected device:

```bash
# Flash the auto-detected firmware artifact
fork flash

# Flash a specific file
fork flash --file ./my-firmware.uf2
```

## Auto-detection

Fork determines which build tool to use by running a `detect_command` in your project directory. For example, it checks whether `embassy-rp` appears in your `Cargo.toml`. The first matching tool is selected. Use `--tool` to override.
