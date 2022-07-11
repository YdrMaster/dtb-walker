# 更新日志 Change Log

此项目的所有显著更改都将记录在此文件中。

格式基于[如何维护更新日志](https://keepachangelog.com/zh-CN/1.0.0/)，本项目遵循[语义化版本](https://semver.org/lang/zh-CN/)。

---

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 未发布

### Changed

- 规范化更新日志格式

---

- standardize the change log

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
