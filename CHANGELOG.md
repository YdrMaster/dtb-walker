# 更新日志 Change Log

## v0.1.2

- 功能
  - 增加一个接收谓词闭包的构造函数，支持忽略某些 `HeaderError`（[issue#1](https://github.com/YdrMaster/dtb-walker/issues/1)）

---

- feature
  - a new function with a filter closure, allows to ignore some `HeaderError` ([issue#1](https://github.com/YdrMaster/dtb-walker/issues/1))

## v0.1.1

- 修正
  - 导出 `HeaderError`

- 示例
  - 演示判断 `HeaderError` 类型以接受某些不合规的首部（[issue#1](https://github.com/YdrMaster/dtb-walker/issues/1)）

---

- fix
  - pub use `HeaderError`

- examples
  - shows the way to allow dtb implemeatations that do not conform to specification by matching the `HeaderError` ([issue#1](https://github.com/YdrMaster/dtb-walker/issues/1))

## v0.1.0

初次发布。

---

First release.
