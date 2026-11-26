# `Trim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

The text without the characters at its ends.

## Syntax

```ahk
Trimmed := Trim(String, OmitChars)
```

## Parameter

- **String** — The text.
- **OmitChars** — Default space+tab.

## Rückgabewert

the text without the leading and the trailing characters of the set; the set is the space and the tab when the caller does not name one

## Fehler

none.

## Bemerkungen

LTrim/RTrim trim one end.

## Beispiel

```ahk
Trim("  hello  ")
```

## Verwandt

[`LTrim`](LTrim.md), [`RTrim`](RTrim.md)
