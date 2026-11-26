# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · L'analyseur et l'exécution du script

## Description

Ends the script with a code. The code goes into the exit_code of the report.

## Syntaxe

```ahk
ExitApp ExitCode
```

## Paramètres

- **ExitCode** — The code the process exits with (default 0).

## Valeur de retour

the code

## Erreurs

none.

## Remarques

OnExit callbacks run before the process ends.

## Exemple

```ahk
ExitApp 3
```

## Voir aussi

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
