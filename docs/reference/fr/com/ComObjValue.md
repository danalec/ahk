# `ComObjValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Reads the underlying pointer/value of a COM wrapper.

## Syntaxe

```ahk
P := ComObjValue(Obj)
```

## Paramètres

- **Obj** — The wrapper.

## Valeur de retour

The pointer or value.

## Exemple

```ahk
p := ComObjValue(o)
```

## Voir aussi

[`ComObjFlags`](ComObjFlags.md), [`ObjPtr`](ObjPtr.md)
