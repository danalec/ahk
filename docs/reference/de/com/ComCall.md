# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · COM-Automatisierung (ComObject)

## Beschreibung

Calls a COM method by its vtable index.

## Syntax

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Parameter

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Rückgabewert

The return value.

## Bemerkungen

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Beispiel

```ahk
ComCall(0, obj, "Ptr")
```

## Verwandt

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
