# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · class · Wywołania funkcji natywnych (DllCall)

## Opis

Allocates a block of bytes.

## Składnia

```ahk
Buf := Buffer(Size, Fill)
```

## Parametry

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Wartość zwracana

A Buffer with Ptr/Size/__Item.

## Przykład

```ahk
b := Buffer(16, 0)
```

## Powiązane

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
