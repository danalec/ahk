# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Składnia

```ahk
Count := StrLen(String)
```

## Parametry

- **String** — The text.

## Wartość zwracana

the number of code units

## Błędy

none; a value that is not text is read as its text.

## Uwagi

Astral characters count as one each on this tree.

## Przykład

```ahk
StrLen("hello")
```

## Powiązane

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
