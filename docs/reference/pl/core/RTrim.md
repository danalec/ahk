# `RTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The text without the characters at its right end.

## Składnia

```ahk
S := RTrim(String, OmitChars)
```

## Parametry

- **String** — The text.
- **OmitChars** — Default space+tab.

## Wartość zwracana

the text without the trailing characters of the set; the set is the space and the tab when the caller does not name one

## Błędy

none.

## Przykład

```ahk
RTrim("  hello  ")
```

## Powiązane

[`Trim`](Trim.md), [`LTrim`](LTrim.md)
