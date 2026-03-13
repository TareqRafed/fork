# Fork

> **Beta software.** Fork is still in early development. If you run into issues, please [open a PR](https://github.com/TareqRafed/fork/pulls).

Fork is a CLI that generates OCI files based on your workspace files and builds your project.

That's possible because `Fork` ships with DSL definitions that describe a tree of toolchain layers, . The matching path accumulates Dockerfile layers and a build command. Fork builds that image, mounts your project directory, and runs the build.

```bash
fork build --mcu rp2040
```

Fork inspects your workspace (e.g. `Cargo.toml`, `rust-toolchain.toml`, `CMakeLists.txt`) and selects the right toolchain automatically. No configuration needed.

---

Built by [@grgo6_](https://x.com/grgo6_) from [simulator86.com](https://simulator86.com)
