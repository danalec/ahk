# `ComObjActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Gets a running COM object by CLSID (the ROT).

## Syntaxe

```ahk
Obj := ComObjActive(CLSID)
```

## Paramètres

- **CLSID** — The class id.

## Valeur de retour

The running object.

## Exemple

```ahk
o := ComObjActive("{...}")
```

## Voir aussi

[`ComObjGet`](ComObjGet.md), [`ComObject`](ComObject.md)
