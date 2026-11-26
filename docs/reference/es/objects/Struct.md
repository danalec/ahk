# `Struct`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*KeySharp* · class · Los objetos del lenguaje: Array, Map y Func

## Descripción

A view of memory at a place, of a type asked for (ks).

## Sintaxis

```ahk
s := Struct(Ptr, Type)
```

## Parámetros

?.

## Valor de retorno

A Struct with At/Size.

## Ejemplo

```ahk
s := Struct(p, "Int")
```

## Relacionados

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
