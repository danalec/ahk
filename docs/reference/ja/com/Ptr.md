# `Ptr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `ComValue` · COM 自動化 (ComObject)

## 説明

The pointer type marker for DllCall/NumGet (the machine's pointer width).

## 構文

```ahk
Ptr
```

## パラメータ

?.

## 戻り値

Used as a type name; 8 bytes on 64-bit, 4 on 32-bit.

## 例

```ahk
NumGet(buf, 0, "Ptr")
```

## 関連

[`PtrSize`](PtrSize.md), [`Buffer`](Buffer.md), [`DllCall`](DllCall.md)
