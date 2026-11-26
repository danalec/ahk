# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Syntax

```ahk
Count := StrLen(String)
```

## Parameter

- **String** — The text.

## Rückgabewert

the number of code units

## Fehler

none; a value that is not text is read as its text.

## Bemerkungen

Astral characters count as one each on this tree.

## Beispiel

```ahk
StrLen("hello")
```

## Verwandt

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
