# `Struct`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*KeySharp* · class · Объекты языка: Array, Map и Func

## Описание

A view of memory at a place, of a type asked for (ks).

## Синтаксис

```ahk
s := Struct(Ptr, Type)
```

## Параметры

?.

## Возвращаемое значение

A Struct with At/Size.

## Пример

```ahk
s := Struct(p, "Int")
```

## См. также

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
