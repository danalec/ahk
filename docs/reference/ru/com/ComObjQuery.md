# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · COM-автоматизация (ComObject)

## Описание

Queries another interface of a COM object.

## Синтаксис

```ahk
Obj := ComObjQuery(Obj, IID)
```

## Параметры

- **Obj** — The object.
- **IID** — The interface.

## Возвращаемое значение

The interface object.

## Пример

```ahk
q := ComObjQuery(o, "{...}")
```

## См. также

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
