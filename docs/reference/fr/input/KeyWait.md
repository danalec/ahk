# `KeyWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Raccourcis, chaînes de déclenchement et saisie

## Description

Refused. The runtime does not keep the keys that are down, so it cannot wait for one.

## Syntaxe

```ahk
KeyWait KeyName, Options
```

## Paramètres

- **KeyName** — The key.
- **Options** — D (down), T timeout, L logical.

## Valeur de retour

nothing; the statement fails

## Erreurs

this runtime does not keep the keys that are down, so it cannot wait for one.

## Exemple

```ahk
KeyWait "a"
```

## Voir aussi

[`GetKeyState`](GetKeyState.md), [`Hotkey`](Hotkey.md)
