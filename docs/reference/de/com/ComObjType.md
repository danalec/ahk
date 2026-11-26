# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · COM-Automatisierung (ComObject)

## Beschreibung

Reads the type/name of a COM object.

## Syntax

```ahk
S := ComObjType(Obj, What)
```

## Parameter

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Rückgabewert

The requested string.

## Beispiel

```ahk
n := ComObjType(o, "Name")
```

## Verwandt

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
