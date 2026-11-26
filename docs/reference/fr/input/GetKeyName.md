# `GetKeyName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Raccourcis, chaînes de déclenchement et saisie

## Description

The name of the key, the way v2 gives it. The names are the ones of the key list of v2: `Esc`, `Enter`, `CtrlRight`, `Numpad0`, and the `vkXX` and `scXXX` spellings.

## Syntaxe

```ahk
GetKeyName …
```

## Paramètres

KeyName

## Valeur de retour

the name of the key

## Erreurs

a name the tree does not know is the empty text, the way v2 answers a name it does not know.

## Exemple

```ahk
name := GetKeyName("Enter")
```
