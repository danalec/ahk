# `ComCall`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · COM automation (ComObject)

## Description

Calls a COM method by its vtable index.

## Syntax

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Parameters

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Return value

The return value.

## Remarks

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Example

```ahk
ComCall(0, obj, "Ptr")
```

## Related

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
