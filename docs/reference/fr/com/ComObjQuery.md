# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Queries another interface of a COM object.

## Syntaxe

```ahk
Obj := ComObjQuery(Obj, IID)
```

## Paramètres

- **Obj** — The object.
- **IID** — The interface.

## Valeur de retour

The interface object.

## Exemple

```ahk
q := ComObjQuery(o, "{...}")
```

## Voir aussi

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
