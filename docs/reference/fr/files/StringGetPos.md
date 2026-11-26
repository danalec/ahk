# `StringGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Les opérations sur les fichiers et les répertoires

## Description

The position of a substring (v1, 0-based).

## Syntaxe

```ahk
StringGetPos Out, Haystack, Needle
```

## Paramètres

3..255.

## Valeur de retour

Nothing; 0-based pos or -1.

## Exemple

```ahk
StringGetPos p, s, "x"
```

## Voir aussi

[`InStr`](InStr.md)
