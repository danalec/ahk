# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The code point of the first character of a text.

## Syntaxe

```ahk
N := Ord(String)
```

## Paramètres

- **String** — The text.

## Valeur de retour

the code point, or zero when the text has no character

## Erreurs

none; a lone surrogate answers with the code unit of it.

## Exemple

```ahk
Ord("A")
```

## Voir aussi

[`Chr`](Chr.md), [`Asc`](Asc.md)
