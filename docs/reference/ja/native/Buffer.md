# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · class · ネイティブ関数の呼び出し (DllCall)

## 説明

Allocates a block of bytes.

## 構文

```ahk
Buf := Buffer(Size, Fill)
```

## パラメータ

- **Size** — Bytes.
- **Fill** — Initial byte value.

## 戻り値

A Buffer with Ptr/Size/__Item.

## 例

```ahk
b := Buffer(16, 0)
```

## 関連

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
