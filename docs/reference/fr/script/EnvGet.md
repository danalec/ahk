# `EnvGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · L'analyseur et l'exécution du script

## Description

Answers the value of an environment variable of the runner; the environment is the one of the process the script runs in.

## Syntaxe

```ahk
V := EnvGet(Name)
```

## Paramètres

- **Name** — The variable.

## Valeur de retour

the value of the variable, or the empty text when the environment does not have it

## Erreurs

nothing; a name that is not there is the empty text and not a failure.

## Exemple

```ahk
home := EnvGet("HOME")
```

## Voir aussi

[`EnvSet`](EnvSet.md)
