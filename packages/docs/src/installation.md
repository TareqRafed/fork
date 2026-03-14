# Installation

## Prerequisites

- **Docker** or **Podman** — required at runtime to build firmware. Fork prefers Podman and falls back to Docker.
  - [Install Docker](https://docs.docker.com/get-docker/)
  - [Install Podman](https://podman.io/get-started)

## Using the install script (recommended)

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/TareqRafed/fork/main/install.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/TareqRafed/fork/main/install.ps1 | iex
```

---

## Pre-built binaries

Download the latest release for your platform from the [GitHub releases page](https://github.com/TareqRafed/fork/releases).

| Platform | Archive |
|----------|---------|
| Linux x86_64 | `fork-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| Linux arm64 | `fork-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| macOS x86_64 | `fork-<version>-x86_64-apple-darwin.tar.gz` |
| macOS arm64 (Apple Silicon) | `fork-<version>-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `fork-<version>-x86_64-pc-windows-msvc.zip` |

**Linux / macOS:**

```bash
tar -xzf fork-<version>-<target>.tar.gz
sudo mv fork /usr/local/bin/
```

**Windows:** extract the `.zip` and place `fork.exe` somewhere on your `PATH`.

---

## From source

Requires [Rust](https://rustup.rs/) (stable toolchain).

```bash
git clone https://github.com/TareqRafed/fork
cd fork
cargo install --path packages/cli
```

The binary is installed to `~/.cargo/bin/fork`. Make sure `~/.cargo/bin` is on your `PATH`.

---

## Verify

```bash
fork --version
```
