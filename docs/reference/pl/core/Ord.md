# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The code point of the first character of a text.

## Składnia

```ahk
N := Ord(String)
```

## Parametry

- **String** — The text.

## Wartość zwracana

the code point, or zero when the text has no character

## Błędy

none; a lone surrogate answers with the code unit of it.

## Przykład

```ahk
Ord("A")
```

## Powiązane

[`Chr`](Chr.md), [`Asc`](Asc.md)
