# `VarSetStrCapacity`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 原生函数调用 (DllCall)

## 描述

Sets the capacity of a variable's string buffer (or frees it).

## 语法

```ahk
VarSetStrCapacity Var, Capacity
```

## 参数

- **Var** — The variable.
- **Capacity** — Chars; -1 frees.

## 返回值

Nothing.

## 示例

```ahk
VarSetStrCapacity s, 1024
```

## 相关

[`StrLen`](StrLen.md), [`Buffer`](Buffer.md)
