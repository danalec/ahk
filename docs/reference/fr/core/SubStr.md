# `SubStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Le cœur du langage : texte, mathématiques et motifs

## Description

The part of a text between two positions. The positions count from one, and a negative position counts from the end: -1 is the last character.

## Syntaxe

```ahk
Part := SubStr(String, Start, Length)
```

## Paramètres

- **String** — The text.
- **Start** — 1-based; negative counts from the end.
- **Length** — Omitted = to the end.

## Valeur de retour

the text of that part; a negative length leaves that many characters out at the end

## Erreurs

none; the text is cut in the code units of UTF-16, so the second unit of a surrogate pair is a text of its own.

## Remarques

Counts characters, not bytes.

## Exemple

```ahk
SubStr("hello", 2, 3)
```

## Voir aussi

[`StrLen`](StrLen.md), [`InStr`](InStr.md), [`StrSplit`](StrSplit.md)
