# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

以一个退出码结束脚本。该退出码进入报告的 exit_code。

## 语法

```ahk
ExitApp ExitCode
```

## 参数

- **ExitCode** — The code the process exits with (default 0).

## 返回值

退出码

## 错误

none.

## 备注

OnExit callbacks run before the process ends.

## 示例

```ahk
ExitApp 3
```

## 相关

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
