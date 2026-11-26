# `Pause`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · L'analyseur et l'exécution du script

## Description

Sets the paused state of the script. The tree keeps a state instead of waiting for a key, which would hang a script that has no keyboard. The state is visible in A_IsPaused.

## Syntaxe

```ahk
Pause …
```

## Paramètres

[OnOff]

## Valeur de retour

the empty text

## Erreurs

a parameter that is not 0, 1 or -1 is an invalid parameter.

## Exemple

```ahk
Pause true
```
