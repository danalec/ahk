# `Object`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Les objets du langage : Array, Map et Func

## Description

The root of the chain of the classes: every object of the runtime is one. `Object()` makes an object with nothing in it, and the literal `{a: 1}` makes one with a property.

## Syntaxe

```ahk
Obj := Object(Props...)
```

## Paramètres

- **Props** — Alternating names and values.

## Valeur de retour

the object of the class `Object`

## Erreurs

nothing; a member that is not there is refused with the type of the value and the name of the member.

## Remarques

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## Exemple

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## Voir aussi

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
