# `RTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The text without the characters at its right end.

## Syntaxe

```ahk
S := RTrim(String, OmitChars)
```

## Paramètres

- **String** — The text.
- **OmitChars** — Default space+tab.

## Valeur de retour

the text without the trailing characters of the set; the set is the space and the tab when the caller does not name one

## Erreurs

none.

## Exemple

```ahk
RTrim("  hello  ")
```

## Voir aussi

[`Trim`](Trim.md), [`LTrim`](LTrim.md)
