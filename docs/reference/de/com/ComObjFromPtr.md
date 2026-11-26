# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · COM-Automatisierung (ComObject)

## Beschreibung

Wraps a COM interface pointer as an object.

## Syntax

```ahk
Obj := ComObjFromPtr(Ptr)
```

## Parameter

- **Ptr** — The interface pointer.

## Rückgabewert

The object.

## Beispiel

```ahk
o := ComObjFromPtr(p)
```

## Verwandt

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
