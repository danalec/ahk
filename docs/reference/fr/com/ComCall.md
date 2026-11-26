# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · L'automatisation COM (ComObject)

## Description

Calls a COM method by its vtable index.

## Syntaxe

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## Paramètres

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## Valeur de retour

The return value.

## Remarques

On Linux the D-Bus model has no vtable index; refuses with the reason.

## Exemple

```ahk
ComCall(0, obj, "Ptr")
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
