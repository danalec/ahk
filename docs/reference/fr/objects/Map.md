# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*KeySharp* · method · `Array` · Les objets du langage : Array, Map et Func

## Description

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## Syntaxe

```ahk
M := Map(Key1, Value1, ...)
```

## Paramètres

- **Pairs** — Alternating keys and values.

## Valeur de retour

the object of the class `Map`

## Erreurs

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## Remarques

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## Exemple

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## Voir aussi

[`Array`](Array.md), [`Object`](Object.md)
