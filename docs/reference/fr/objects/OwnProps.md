# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · method · `Object` · Les objets du langage : Array, Map et Func

## Description

Iterates the own properties of an object.

## Syntaxe

```ahk
for name, value in Obj.OwnProps()
```

## Paramètres

?.

## Valeur de retour

The enumerator.

## Exemple

```ahk
for n, v in o.OwnProps()
```

## Voir aussi

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
