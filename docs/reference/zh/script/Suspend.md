# `Suspend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

关闭或打开脚本的热键。不带参数时切换状态。状态可在 A_IsSuspended 和热键表的启用标志中看到。

## 语法

```ahk
Suspend …
```

## 参数

[OnOff]

## 返回值

空文本

## 错误

a parameter that is not 0, 1 or -1 is an invalid parameter.

## 示例

```ahk
Suspend true
```
