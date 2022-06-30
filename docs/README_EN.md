# DTB depth-first walking

- [中文自述文档](../README.md)
- [Change Log](../CHANGELOG.md)

A simple package for DTB depth-first walking.

Try an example:

```cmd
cargo run --release --example qemu-virt
```

Following the [devicetree-specification-v0.4-rc1](https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.4-rc1)，DTB v17。

Features:

- [x] stable rust
- [x] `no_std`;
- [x] without `alloc`;
- [x] optional header verifying;
- [x] terminate walking at any time;
- [x] step over nodes with low overhead;
- [ ] built-in standard property parsing;
  - [x] `compatible`
  - [x] `model`
  - [x] `phandle`
  - [x] `status`
  - [x] `#address-cells`
  - [x] `#size-cells`
  - [x] `reg`
  - [x] `virtual-reg`
  - [ ] `ranges`
  - [ ] `dma-ranges`
  - [x] `dma-coherent`
  - [ ] `name (deprecated)`
  - [ ] `device_type (deprecated)`
