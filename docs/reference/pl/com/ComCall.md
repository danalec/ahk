# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Automatyzacja COM (ComObject)

## Opis

Calls a COM method by its vtable index.

## Składnia

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Parametry

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Wartość zwracana

The return value.

## Uwagi

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Przykład

```ahk
ComCall(0, obj, "Ptr")
```

## Powiązane

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
