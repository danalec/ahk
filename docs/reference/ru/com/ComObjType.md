# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · COM-автоматизация (ComObject)

## Описание

Reads the type/name of a COM object.

## Синтаксис

```ahk
S := ComObjType(Obj, What)
```

## Параметры

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Возвращаемое значение

The requested string.

## Пример

```ahk
n := ComObjType(o, "Name")
```

## См. также

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
