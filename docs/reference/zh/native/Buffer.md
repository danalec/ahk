# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · class · 原生函数调用 (DllCall)

## 描述

Allocates a block of bytes.

## 语法

```ahk
Buf := Buffer(Size, Fill)
```

## 参数

- **Size** — Bytes.
- **Fill** — Initial byte value.

## 返回值

A Buffer with Ptr/Size/__Item.

## 示例

```ahk
b := Buffer(16, 0)
```

## 相关

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
