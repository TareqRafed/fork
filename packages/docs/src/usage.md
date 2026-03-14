# Usage

## Commands


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

