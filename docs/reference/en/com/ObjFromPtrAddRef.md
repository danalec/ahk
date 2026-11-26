# `ObjFromPtrAddRef`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · COM automation (ComObject)

## Description

Wraps an object pointer and adds a reference.

## Syntax

```ahk
Obj := ObjFromPtrAddRef(Ptr)
```

## Parameters

- **Ptr** — The pointer.

## Return value

The object with a reference.

## Example

```ahk
o := ObjFromPtrAddRef(p)
```

## Related

[`ObjFromPtr`](ObjFromPtr.md), [`ObjRelease`](ObjRelease.md)
