# `OnExit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · L'analyseur et l'exécution du script

## Description

Registers the function that runs when the script ends, whatever ended it: the last statement, a code the script asked to stop with, or a failure that stopped it.

## Syntaxe

```ahk
OnExit Function, Add
```

## Paramètres

- **Function** — The callback (ExitReason, ExitCode).
- **Add** — 1 to add, -1 to remove.

## Valeur de retour

the integer 1

## Erreurs

a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

## Exemple

```ahk
OnExit((reason, code) => 0)
```

## Voir aussi

[`ExitApp`](ExitApp.md), [`OnError`](OnError.md)
