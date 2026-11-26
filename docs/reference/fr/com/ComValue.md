# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · L'automatisation COM (ComObject)

## Description

Wraps a raw value as a VARIANT of a given type.

## Syntaxe

```ahk
V := ComValue(Type, Value)
```

## Paramètres

- **Type** — The VARTYPE.
- **Value** — The value.

## Valeur de retour

A COM value.

## Exemple

```ahk
v := ComValue(0x4000, 1)
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
