# `Buffer`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · class · Llamadas a funciones nativas (DllCall)

## Descripción

Allocates a block of bytes.

## Sintaxis

```ahk
Buf := Buffer(Size, Fill)
```

## Parámetros

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Valor de retorno

A Buffer with Ptr/Size/__Item.

## Ejemplo

```ahk
b := Buffer(16, 0)
```

## Relacionados

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
