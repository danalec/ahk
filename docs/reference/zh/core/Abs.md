# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

与零的距离。整数仍然是整数。

## 语法

```ahk
N := Abs(Number)
```

## 参数

- **Number** — Integer or float.

## 返回值

去掉符号后的数字

## 错误

a value that is not a number is refused.

## 备注

Scalar; no SIMD gain (docs/simd.md).

## 示例

```ahk
Abs(-3)
```

## 相关

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
