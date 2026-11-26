# `RTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

The text without the characters at its right end.

## Syntax

```ahk
S := RTrim(String, OmitChars)
```

## Parameter

- **String** — The text.
- **OmitChars** — Default space+tab.

## Rückgabewert

the text without the trailing characters of the set; the set is the space and the tab when the caller does not name one

## Fehler

none.

## Beispiel

```ahk
RTrim("  hello  ")
```

## Verwandt

[`Trim`](Trim.md), [`LTrim`](LTrim.md)
