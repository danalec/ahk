# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · class · COM-автоматизация (ComObject)

## Описание

Wraps a raw value as a VARIANT of a given type.

## Синтаксис

```ahk
V := ComValue(Type, Value)
```

## Параметры

- **Type** — The VARTYPE.
- **Value** — The value.

## Возвращаемое значение

A COM value.

## Пример

```ahk
v := ComValue(0x4000, 1)
```

## См. также

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
