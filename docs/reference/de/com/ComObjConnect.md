# `ComObjConnect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · COM-Automatisierung (ComObject)

## Beschreibung

Connects a COM object's events to script functions.

## Syntax

```ahk
ComObjConnect Obj, Prefix
```

## Parameter

- **Obj** — The object.
- **Prefix** — The function prefix.

## Rückgabewert

Nothing; the event sink refuses on Linux.

## Beispiel

```ahk
ComObjConnect ie, "IE_"
```

## Verwandt

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
