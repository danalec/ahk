# `Buffer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · class · Вызовы нативных функций (DllCall)

## Описание

Allocates a block of bytes.

## Синтаксис

```ahk
Buf := Buffer(Size, Fill)
```

## Параметры

- **Size** — Bytes.
- **Fill** — Initial byte value.

## Возвращаемое значение

A Buffer with Ptr/Size/__Item.

## Пример

```ahk
b := Buffer(16, 0)
```

## См. также

[`NumGet`](NumGet.md), [`NumPut`](NumPut.md), [`StrGet`](StrGet.md)
