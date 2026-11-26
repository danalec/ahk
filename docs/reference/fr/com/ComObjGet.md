# `ComObjGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Gets a COM object from a moniker (a file/URL/ProgID).

## Syntaxe

```ahk
Obj := ComObjGet(Name)
```

## Paramètres

- **Name** — The moniker.

## Valeur de retour

The object.

## Exemple

```ahk
o := ComObjGet("winmgmts:")
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComObjActive`](ComObjActive.md)
