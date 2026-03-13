# Supported Boards

Each board ships with definitions for the toolchains listed below. Fork auto-selects the right one based on your workspace files.

## RP2040

| Recipe | Detected by |
|--------|-------------|
| `rust → cargo → rustc → thumbv6mnonenabi` | `Cargo.toml` + `.cargo/config.toml` with `thumbv6m-none-eabi` |
| `c → cmake → gcc → armnoneabi → picosdk` | `CMakeLists.txt` with `pico_sdk` |
| `arduino` | `*.ino` file |
| `micropython` | `main.py` with `import machine` |
| `circuitpython` | `code.py` with `import board` |

## ESP32-C3

| Recipe | Detected by |
|--------|-------------|
| `rust → cargo → rustc → riscv32imcacpnoneelf → esphal` | `Cargo.toml` with `esp-hal` |
| `rust → cargo → rustc → riscv32imcacpnoneelf → espidf` | `Cargo.toml` with `esp-idf-svc` |
| `c → cmake → gcc → riscv32espelf → espidf` | `sdkconfig` with `CONFIG_IDF_TARGET_ESP32C3` |
| `arduino` | `*.ino` file |
| `micropython` | `main.py` |

## ESP32-S3

| Recipe | Detected by |
|--------|-------------|
| `rust → cargo → rustc → xtensaesp32s3noneelf → esphal` | `Cargo.toml` with `esp-hal` |
| `rust → cargo → rustc → xtensaesp32s3noneelf → espidf` | `Cargo.toml` with `esp-idf-svc` |
| `c → cmake → gcc → xtensaesp32s3elfelf → espidf` | `sdkconfig` with `CONFIG_` |
| `arduino` | `*.ino` file |
| `micropython` | `main.py` with `import machine` |
| `circuitpython` | `code.py` with `import board` |

## STM32F405

| Recipe | Detected by |
|--------|-------------|
| `rust → cargo → rustc → thumbv7emnoneeabihf` | `Cargo.toml` + `.cargo/config.toml` with `thumbv7em-none-eabihf` |
| `c → cmake → gcc → armnoneabi → stm32cube` | `CMakeLists.txt` with `stm32` |
| `arduino` | `*.ino` file |

## STM32F103

| Recipe | Detected by |
|--------|-------------|
| `rust → cargo → rustc → thumbv7mnonenabi` | `Cargo.toml` + `.cargo/config.toml` with `thumbv7m-none-eabi` |
| `c → cmake → gcc → armnoneabi → stm32cube` | `CMakeLists.txt` with `stm32` |
| `arduino` | `*.ino` file |
