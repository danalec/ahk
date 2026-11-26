# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Syntaxe

```ahk
Count := StrLen(String)
```

## Paramètres

- **String** — The text.

## Valeur de retour

the number of code units

## Erreurs

none; a value that is not text is read as its text.

## Remarques

Astral characters count as one each on this tree.

## Exemple

```ahk
StrLen("hello")
```

## Voir aussi

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
