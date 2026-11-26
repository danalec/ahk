# `ObjPtrAddRef`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · COM automation (ComObject)

## Description

The pointer of an object, adding a reference.

## Syntax

```ahk
P := ObjPtrAddRef(Obj)
```

## Parameters

- **Obj** — The object.

## Return value

The pointer with a reference.

## Example

```ahk
p := ObjPtrAddRef(o)
```

## Related

[`ObjPtr`](ObjPtr.md), [`ObjRelease`](ObjRelease.md)
