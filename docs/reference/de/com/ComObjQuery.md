# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · COM-Automatisierung (ComObject)

## Beschreibung

Queries another interface of a COM object.

## Syntax

```ahk
Obj := ComObjQuery(Obj, IID)
```

## Parameter

- **Obj** — The object.
- **IID** — The interface.

## Rückgabewert

The interface object.

## Beispiel

```ahk
q := ComObjQuery(o, "{...}")
```

## Verwandt

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
