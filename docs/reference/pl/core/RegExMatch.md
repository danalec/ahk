# `RegExMatch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

Where a pattern is in a text. The position counts from one and is zero when the pattern is not there.

## Składnia

```ahk
RegExMatch …
```

## Parametry

Text, Pattern, [&Output], [Start]

## Wartość zwracana

the position of the match, or 0 when there is none

## Błędy

the output must be a variable reference; the output holds an object of the class RegExMatchInfo, and a text that does not match leaves it holding the empty text.

## Przykład

```ahk
RegExMatch("abc123", "\d+")
```
