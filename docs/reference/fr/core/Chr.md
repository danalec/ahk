# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The character with a code point.

## Syntaxe

```ahk
S := Chr(Number)
```

## Paramètres

- **Number** — The code point.

## Valeur de retour

the text of that character

## Erreurs

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Exemple

```ahk
Chr(65)
```

## Voir aussi

[`Ord`](Ord.md), [`Asc`](Asc.md)
