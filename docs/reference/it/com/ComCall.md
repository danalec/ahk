# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Automazione COM (ComObject)

## Descrizione

Calls a COM method by its vtable index.

## Sintassi

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Parametri

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Valore di ritorno

The return value.

## Osservazioni

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Esempio

```ahk
ComCall(0, obj, "Ptr")
```

## Correlati

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
