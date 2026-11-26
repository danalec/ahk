# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Reads the type/name of a COM object.

## Syntaxe

```ahk
S := ComObjType(Obj, What)
```

## Paramètres

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Valeur de retour

The requested string.

## Exemple

```ahk
n := ComObjType(o, "Name")
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
