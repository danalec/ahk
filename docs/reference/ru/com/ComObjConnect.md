# `ComObjConnect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · COM-автоматизация (ComObject)

## Описание

Connects a COM object's events to script functions.

## Синтаксис

```ahk
ComObjConnect Obj, Prefix
```

## Параметры

- **Obj** — The object.
- **Prefix** — The function prefix.

## Возвращаемое значение

Nothing; the event sink refuses on Linux.

## Пример

```ahk
ComObjConnect ie, "IE_"
```

## См. также

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
