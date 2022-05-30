# 深度优先遍历设备树二进制对象

DTB 深度优先遍历的薄封装。

设备树定义根据 [devicetree-specification-v0.4-rc1](https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.4-rc1)，DTB v17。

特性：

- [x] 可选是否检查首部正确性；
- [x] `no_std`；
- [x] without `alloc`；
- [x] 提前终止遍历；
- [x] 低开销跳过节点；
- [ ] 内置标准属性解析；
  - [ ] `compatible`
  - [ ] `model`
  - [ ] `phandle`
  - [ ] `status`
  - [x] `#address-cells`
  - [x] `#size-cells`
  - [x] `reg`
  - [ ] `virtual-reg`
  - [ ] `ranges`
  - [ ] ...
