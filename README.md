# 深度优先遍历设备树二进制对象

[![Build status](https://img.shields.io/github/workflow/status/YdrMaster/dtb-walker/CI/main)](https://github.com/YdrMaster/dtb-walker/actions)
[![Latest version](https://img.shields.io/crates/v/dtb-walker.svg)](https://crates.io/crates/dtb-walker)
[![Documentation](https://docs.rs/dtb-walker/badge.svg)](https://docs.rs/dtb-walker)
![License](https://img.shields.io/crates/l/dtb-walker.svg)

- [An English README](docs/README_EN.md)
- [更新日志](CHANGELOG.md)

DTB 深度优先遍历的薄封装。

测试示例：

```cmd
cargo run --release --example qemu-virt
```

设备树定义根据 [devicetree-specification-v0.4-rc1](https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.4-rc1)，DTB v17。

特性：

- [x] stable rust
- [x] `no_std`；
- [x] 不需要 `alloc`；
- [x] 可选是否检查首部正确性；
- [x] 提前终止遍历；
- [x] 低开销跳过节点；
- [ ] 内置标准属性解析；
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
