# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The character with a code point.

## Składnia

```ahk
S := Chr(Number)
```

## Parametry

- **Number** — The code point.

## Wartość zwracana

the text of that character

## Błędy

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Przykład

```ahk
Chr(65)
```

## Powiązane

[`Ord`](Ord.md), [`Asc`](Asc.md)
