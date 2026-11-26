# `Struct`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*KeySharp* · class · The objects of the language: Array, Map and Func

## Description

A view of memory at a place, of a type asked for (ks).

## Syntax

```ahk
s := Struct(Ptr, Type)
```

## Parameters

?.

## Return value

A Struct with At/Size.

## Example

```ahk
s := Struct(p, "Int")
```

## Related

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
