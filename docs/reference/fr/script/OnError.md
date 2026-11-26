# `OnError`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'analyseur et l'exécution du script

## Description

Registers the function that handles a failure the script did not catch.

## Syntaxe

```ahk
OnError Function, Add
```

## Paramètres

- **Function** — The callback (Thrown, Mode).
- **Add** — 1/-1.

## Valeur de retour

the integer 1

## Erreurs

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Exemple

```ahk
OnError((err, mode) => 0)
```

## Voir aussi

[`OnExit`](OnExit.md), [`Throw`](Throw.md)
