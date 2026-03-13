# Adding a Board

Fork the repo and create a TOML file in the `boards/` directory at the root of the repository:

```toml
name = "your-board"


[[build_tools]]
name = "some-hal"
docker_image = "rust:latest"
build_command = ["cargo", "build", "--release", "--target", "thumbv7em-none-eabihf"]
artifact_path = "target/thumbv7em-none-eabihf/release/firmware.bin"
detect_command = "grep -q 'some-hal' Cargo.toml"
```

## Fields

### Top-level

| Field | Description |
|-------|-------------|
| `name` | Unique board identifier |
| `flash_tool` | Tool used to flash the device (e.g. `elf2uf2-rs`, `espflash`, `dfu-util`) |

### `[usb]`

| Field | Description |
|-------|-------------|
| `vid` | USB Vendor ID (hex) |
| `pid` | USB Product ID(s) — single value or list |

### `[[build_tools]]`

Each board can have multiple build tools. Fork selects one via auto-detection or the `--tool` flag.

| Field | Description |
|-------|-------------|
| `name` | Build tool identifier (used with `--tool`) |
| `docker_image` | Docker image to run the build in |
| `build_command` | Command to run inside the container |
| `artifact_path` | Path to the built firmware artifact, relative to the project root |
| `detect_command` | Shell command run in the project directory to check compatibility — exit 0 means compatible |

