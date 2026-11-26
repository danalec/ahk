# `RegExReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The text with the matches of a pattern replaced. The replacement may name the groups of the pattern with $1 or \1, and $$ is a dollar of its own.

## Składnia

```ahk
RegExReplace …
```

## Parametry

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## Wartość zwracana

the text with the replacements

## Błędy

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## Przykład

```ahk
RegExReplace("a1b2", "\d", "#")
```
