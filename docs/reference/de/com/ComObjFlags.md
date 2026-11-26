# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · COM-Automatisierung (ComObject)

## Beschreibung

Gets/sets the flags of a COM wrapper.

## Syntax

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## Parameter

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## Rückgabewert

The flags.

## Beispiel

```ahk
f := ComObjFlags(o)
```

## Verwandt

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
