# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · Les objets du langage : Array, Map et Func

## Description

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## Syntaxe

```ahk
Arr := Array(Values...)
```

## Paramètres

- **Values** — The initial elements.

## Valeur de retour

the object of the class `Array`

## Erreurs

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## Remarques

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## Exemple

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## Voir aussi

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
