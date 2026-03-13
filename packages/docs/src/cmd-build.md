# fork build

Builds your firmware inside the appropriate OCI container.

Fork inspects your workspace, resolves the matching toolchain path from the board definition, builds (or pulls) the Docker image, and runs the build command with your project directory mounted.

## Usage

```bash
fork build [OPTIONS] [PATH] [-- EXTRA...]
```

## Options

| Flag | Description |
|------|-------------|
| `-m, --mcu <name>` | Target MCU (e.g. `rp2040`, `esp32c3`). If omitted, you will be prompted to select one. |
| `-t, --tool <name>` | Force a specific recipe by label (e.g. `thumbv6mnonenabi`). Skips auto-detection. |
| `-r, --registry <url>` | Registry prefix to pull images from (e.g. `ghcr.io/your-org`). If omitted, images are built locally. |
| `[PATH]` | Path to the workspace root. Defaults to `.` |
| `-- <EXTRA>` | Extra arguments passed through to the build command. |

## Examples

```bash
# Auto-detect toolchain from workspace
fork build --mcu rp2040

# Pass extra flags to the build system
fork build --mcu rp2040 -- --release

# Force a specific recipe
fork build --mcu rp2040 --tool thumbv6mnonenabi

# Build from a different directory
fork build --mcu esp32c3 ./firmware/esp32

# Use images from a registry instead of building locally
fork build --mcu rp2040 --registry ghcr.io/your-org
```

## How Detection Works

Fork walks the board's toolchain tree and runs each node's `detect` rules against your workspace. The first fully matching path wins. Use `--tool` to override this and select a recipe by the last segment of its label.

If no toolchain matches, Fork exits with an error listing the available recipes for that board.
