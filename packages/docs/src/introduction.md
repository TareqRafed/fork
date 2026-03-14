# Fork

> **Beta software.** Fork is still in early development. If you run into issues, please [open a PR](https://github.com/TareqRafed/fork/pulls).

Fork analyzes your workspace then generates a container to build your project. Fork defaults to [podman]() then docker if podman isn't found.

Fork doesn't require any dotfiles, pre-installed toolchain (except of OCI runner, like docker or podman), configuration or any sort of file system, it works completely from CLI and builds your project in an isolated environment.

What a typical recipe do:

* Inspect your source code to detect the programming language, version, compiler and target
* Install programming language binaries
* Install dependencies
* Compile code


Example:  

```bash
# Ensure docker/podman already installed
fork build --mcu rp2040
```

Outputs:

```bash
fork build -c rp2040

# Outputs

fork — container orchestration for firmware


→ Using docker  
✓ Toolchain: rust → cargo → rustc → thumbv6m-none-eabi  
→ Ensuring image fork-local/rp2040/rust.cargo.rustc.thumbv6m-none-eabi:latest  

[+] Building 0.2s (7/7) FINISHED
docker:default

=> [internal] load build definition from Dockerfile
=> => transferring dockerfile: 122B
=> [internal] load metadata for docker.io/library/rust:latest
=> [internal] load .dockerignore
=> => transferring context: 2B
=> [1/3] FROM docker.io/library/rust:latest
=> CACHED [2/3] RUN rustup target add thumbv6m-none-eabi
=> CACHED [3/3] RUN cargo install flip-link
=> exporting to image
=> => exporting layers
=> => writing image sha256:d6b43c5fe1f26f23498f68424c84d29b28ccea8f71b6449f02a17
=> => naming to docker.io/fork-local/rp2040/rust.cargo.rustc.thumbv6m-none-eabi:latest

→ Building rp2040 with rust → cargo → rustc → thumbv6m-none-eabi
──────────────────────────────────────────────────

Updating crates.io index
Downloading crates ...

Downloaded bare-metal v0.2.5
.
.
.
Downloaded rp2040-pac v0.6.0

Finished `dev` profile [optimized + debuginfo] target(s) in 2.09s

──────────────────────────────────────────────────
✓ Build complete.
    
```

That's all

## Why?

If you work with multiple MCUs, you'll need to set up different tools and face version conflicts or clutter your machine with unnecessary issues. It's not aimed to replace custom workflows that use multiple languages or custom linker setups, but if you want to onboard a team member, I don't know any faster way to do so.

Originally it was designed as a building block for an IDE extension for a custom board with different variations. Maintaining multiple images would have made the maintenance expensive, so the approach taken to solve that is to have branching ordered in a tree-like model to resolve the workspace toolchain. Similar tool used by webserver providers: [buildpack by heroku](https://devcenter.heroku.com/articles/buildpacks).




---

Built by [@grgo6_](https://x.com/grgo6_) from [simulator86.com](https://simulator86.com)
