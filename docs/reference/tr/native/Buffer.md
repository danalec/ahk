# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · class · Yerel işlev çağrıları (DllCall)

## Açıklama

Allocates a block of bytes.

## Sözdizimi

```ahk
Buf := Buffer(Size, Fill)
```

## Parametreler

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Dönüş değeri

A Buffer with Ptr/Size/__Item.

## Örnek

```ahk
b := Buffer(16, 0)
```

## İlgili

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
