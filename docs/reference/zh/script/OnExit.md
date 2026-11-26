# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 解析器与脚本执行

## 描述

注册脚本结束时运行的函数，无论结束的原因是什么：最后一条语句、脚本请求停止的退出码，或让脚本停下的失败。

## 语法

```ahk
OnExit Function, Add
```

## 参数

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## 返回值

整数 1

## 错误

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## 示例

```ahk
OnExit((reason, code) => 0)
```

## 相关

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
