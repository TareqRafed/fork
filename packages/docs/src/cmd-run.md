# fork run

Runs an arbitrary command inside the MCU's container.

Same image resolution as `build` — Fork detects the toolchain from your workspace and spins up the container — but instead of the default build command, it runs whatever you pass.

## Usage

```bash
fork run [OPTIONS] [PATH] <COMMAND>
```

## Options

| Flag | Description |
|------|-------------|
| `-m, --mcu <name>` | Target MCU. If omitted, you will be prompted to select one. |
| `[PATH]` | Path to the workspace root. Defaults to `.` |
| `<COMMAND>` | Command to run inside the container. |

## Examples

```bash
# Run cargo check inside the rp2040 container
fork run --mcu rp2040 "cargo check"

# Open a shell for debugging
fork run --mcu esp32c3 "bash"

# Run from a specific directory
fork run --mcu stm32f405 ./firmware "cargo clippy"
```
