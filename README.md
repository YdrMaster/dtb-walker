# 深度优先遍历设备树二进制对象

[![CI](https://github.com/YdrMaster/dtb-walker/actions/workflows/workflow.yml/badge.svg?branch=main)](https://github.com/YdrMaster/dtb-walker/actions)
[![Latest version](https://img.shields.io/crates/v/dtb-walker.svg)](https://crates.io/crates/dtb-walker)
[![issue](https://img.shields.io/github/issues/YdrMaster/dtb-walker)](https://github.com/YdrMaster/dtb-walker/issues)
[![Documentation](https://docs.rs/dtb-walker/badge.svg)](https://docs.rs/dtb-walker)
![license](https://img.shields.io/github/license/YdrMaster/dtb-walker)

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
- [x] 警告视为错误（包括 clippy）
- [x] 零开销抽象
  - [x] `no_std`
  - [x] 不需要 `alloc`
  - [x] 可选是否检查首部正确性
  - [x] 提前终止遍历
  - [x] 标记跳过的节点不解析
- [x] 内置标准属性解析
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
