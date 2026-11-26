# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · class · Chiamate a funzioni native (DllCall)

## Descrizione

Allocates a block of bytes.

## Sintassi

```ahk
Buf := Buffer(Size, Fill)
```

## Parametri

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Valore di ritorno

A Buffer with Ptr/Size/__Item.

## Esempio

```ahk
b := Buffer(16, 0)
```

## Correlati

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
