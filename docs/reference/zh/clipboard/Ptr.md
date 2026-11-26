# `Ptr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `ClipboardAll` · 剪贴板

## 描述

The pointer type marker for DllCall/NumGet (the machine's pointer width).

## 语法

```ahk
Ptr
```

## 参数

?.

## 返回值

Used as a type name; 8 bytes on 64-bit, 4 on 32-bit.

## 示例

```ahk
NumGet(buf, 0, "Ptr")
```

## 相关

[`PtrSize`](PtrSize.md), [`Buffer`](Buffer.md), [`DllCall`](DllCall.md)
