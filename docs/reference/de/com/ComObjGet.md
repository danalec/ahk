# `ComObjGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · COM-Automatisierung (ComObject)

## Beschreibung

Gets a COM object from a moniker (a file/URL/ProgID).

## Syntax

```ahk
Obj := ComObjGet(Name)
```

## Parameter

- **Name** — The moniker.

## Rückgabewert

The object.

## Beispiel

```ahk
o := ComObjGet("winmgmts:")
```

## Verwandt

[`ComObject`](ComObject.md), [`ComObjActive`](ComObjActive.md)
