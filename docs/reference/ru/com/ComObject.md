# `ComObject`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · class · COM-автоматизация (ComObject)

## Описание

Creates or gets a COM object (Windows).

## Синтаксис

```ahk
Obj := ComObject(CLSID, IID)
```

## Параметры

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## Возвращаемое значение

A COM object.

## Замечания

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## Пример

```ahk
shell := ComObject("Shell.Application")
```

## См. также

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
