# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · class · 原生函式呼叫 (DllCall)

## 描述

Allocates a block of bytes.

## 語法

```ahk
Buf := Buffer(Size, Fill)
```

## 參數

- **Size** — Bytes.
- **Fill** — Initial byte value.

## 傳回值

A Buffer with Ptr/Size/__Item.

## 範例

```ahk
b := Buffer(16, 0)
```

## 相關

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
