# `ComObjConnect`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · COM automation (ComObject)

## Description

Connects a COM object's events to script functions.

## Syntax

```ahk
ComObjConnect Obj, Prefix
```

## Parameters

- **Obj** — The object.
- **Prefix** — The function prefix.

## Return value

Nothing; the event sink refuses on Linux.

## Example

```ahk
ComObjConnect ie, "IE_"
```

## Related

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
