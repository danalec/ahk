# `ObjBindMethod`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · COM automation (ComObject)

## Description

Binds an object to a method, answering a BoundFunc.

## Syntax

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Parameters

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Return value

A BoundFunc.

## Example

```ahk
f := ObjBindMethod(o, "m", 1)
```

## Related

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
