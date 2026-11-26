# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Gets/sets the flags of a COM wrapper.

## Syntaxe

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## Paramètres

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## Valeur de retour

The flags.

## Exemple

```ahk
f := ComObjFlags(o)
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
