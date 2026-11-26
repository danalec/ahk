# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## Syntaxe

```ahk
R := Mod(Dividend, Divisor)
```

## Paramètres

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Valeur de retour

the remainder

## Erreurs

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Remarques

A zero divisor throws ZeroDivisionError.

## Exemple

```ahk
Mod(7, 3)
```

## Voir aussi

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
