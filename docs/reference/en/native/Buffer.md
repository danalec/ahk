# `Buffer`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · class · Native function calls (DllCall)

## Description

Allocates a block of bytes.

## Syntax

```ahk
Buf := Buffer(Size, Fill)
```

## Parameters

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Return value

A Buffer with Ptr/Size/__Item.

## Example

```ahk
b := Buffer(16, 0)
```

## Related

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
