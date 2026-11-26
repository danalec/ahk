# `ComObjGet`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · COM automation (ComObject)

## Description

Gets a COM object from a moniker (a file/URL/ProgID).

## Syntax

```ahk
Obj := ComObjGet(Name)
```

## Parameters

- **Name** — The moniker.

## Return value

The object.

## Example

```ahk
o := ComObjGet("winmgmts:")
```

## Related

[`ComObject`](ComObject.md), [`ComObjActive`](ComObjActive.md)
