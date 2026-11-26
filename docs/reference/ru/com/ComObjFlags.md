# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · COM-автоматизация (ComObject)

## Описание

Gets/sets the flags of a COM wrapper.

## Синтаксис

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## Параметры

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## Возвращаемое значение

The flags.

## Пример

```ahk
f := ComObjFlags(o)
```

## См. также

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
