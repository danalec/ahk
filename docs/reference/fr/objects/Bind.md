# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · method · `Func` · Les objets du langage : Array, Map et Func

## Description

Binds arguments to a function, answering a BoundFunc.

## Syntaxe

```ahk
F := Func.Bind(Args...)
```

## Paramètres

?.

## Valeur de retour

A BoundFunc.

## Exemple

```ahk
f := Func.Bind(1)
```

## Voir aussi

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
