# fork bake

Builds and pushes all images for a board to a container registry.

`bake` iterates over every recipe in the board definition (ignoring `detect` rules — it bakes all of them), builds each image, and pushes it to the given registry. This is intended for CI pipelines that pre-build images so end users can pull instead of building locally.

## Usage

```bash
fork bake [OPTIONS] [PATH]
```

## Options

| Flag | Description |
|------|-------------|
| `-c, --recipe <name>` | Target recipe. If omitted, you will be prompted to select one. |
| `-r, --registry <url>` | Registry prefix to push images to (e.g. `ghcr.io/your-org`). **Required.** |
| `[PATH]` | Path to the workspace root. Defaults to `.` |

## Image Tagging

Images are tagged as:

```
{registry}/{recipe-name}/{recipe-label}:{version}
```

The version is extracted from the `FROM` line in the generated Dockerfile. For example:

```
ghcr.io/your-org/rp2040/rust.cargo.rustc.thumbv6mnonenabi:1.76
```

## Examples

```bash
# Bake all rp2040 images and push to a registry
fork bake --recipe rp2040 --registry ghcr.io/your-org

# Bake all boards in CI
fork bake --recipe esp32c3 --registry ghcr.io/your-org
fork bake --recipe stm32f405 --registry ghcr.io/your-org
```

Once images are pushed, users can pull them with `--registry` on `fork build`:

```bash
fork build --recipe rp2040 --registry ghcr.io/your-org
```
