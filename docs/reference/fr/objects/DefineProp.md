# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · method · `Object` · Les objets du langage : Array, Map et Func

## Description

Defines a property with a getter/setter or value.

## Syntaxe

```ahk
Obj.DefineProp(Name, Desc)
```

## Paramètres

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Valeur de retour

Nothing.

## Exemple

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## Voir aussi

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
