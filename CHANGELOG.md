# 更新日志 Change Log

此项目的所有显著更改都将记录在此文件中。

格式基于[如何维护更新日志](https://keepachangelog.com/zh-CN/1.0.0/)，本项目遵循[语义化版本](https://semver.org/lang/zh-CN/)。

---

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 未发布

## Unreleased

## [0.2.0-alpha.1](https://github.com/YdrMaster/dtb-walker/releases/tag/0.2.0-alpha.1) - 2022-07-12

- 规范化更新日志格式
- 字符串统一使用一个封装的 `Str` 类型（包括节点名、属性名、`<string>` 类型的属性值、路径），类似于 `str` 但未检查是否符合 utf-8 编码
- 格式化 `Str` 不再自带引号
- 补全文档并禁止不写文档
- github ci 自动运行一次示例

---

- standardizes the change log
- uses an encapsulated `Str` type uniformly for strings (including node name, property name, property value of `<string>`, path), similar to `str` but not checked for utf-8 encoding
- will not add quotes when formating `Str`
- completes documentation and missing documentation is denied from now on
- runs example during github ci

## [0.1.3](https://github.com/YdrMaster/dtb-walker/releases/tag/v0.1.3) - 2022-06-30

### Changed

- 移除不稳定特性，支持 stable 编译

---

- removes unstable features and allows to compile with stable rust

## [0.1.2](https://github.com/YdrMaster/dtb-walker/releases/tag/v0.1.2) - 2022-06-30

### Added

- 增加一个接收谓词闭包的构造函数，支持忽略某些 `HeaderError`（[issue#1](https://github.com/YdrMaster/dtb-walker/issues/1)）

---

- a new function with a filter closure, allows to ignore some `HeaderError` ([issue#1](https://github.com/YdrMaster/dtb-walker/issues/1))

## [0.1.1](https://github.com/YdrMaster/dtb-walker/releases/tag/v0.1.1) - 2022-06-18

### Fixed

- 导出 `HeaderError`

---

- pub use `HeaderError`

### Added

- 演示判断 `HeaderError` 类型以接受某些不合规的首部（[issue#1](https://github.com/YdrMaster/dtb-walker/issues/1)）

---

- shows the way to allow dtb implemeatations that do not conform to specification by matching the `HeaderError` ([issue#1](https://github.com/YdrMaster/dtb-walker/issues/1))

## 0.1.0 - 2022-05-30

初次发布。

---

First release.
