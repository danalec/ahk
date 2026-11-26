# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The distance from zero. An integer stays an integer.

## Syntaxe

```ahk
N := Abs(Number)
```

## Paramètres

- **Number** — Integer or float.

## Valeur de retour

the number without its sign

## Erreurs

a value that is not a number is refused.

## Remarques

Scalar; no SIMD gain (docs/simd.md).

## Exemple

```ahk
Abs(-3)
```

## Voir aussi

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
