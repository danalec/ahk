# `ComObjFlags`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · COM automation (ComObject)

## Description

Gets/sets the flags of a COM wrapper.

## Syntax

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## Parameters

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## Return value

The flags.

## Example

```ahk
f := ComObjFlags(o)
```

## Related

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
