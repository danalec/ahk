# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*KeySharp* · method · `Array` · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## Składnia

```ahk
S := Sort(String, Options)
```

## Parametry

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## Wartość zwracana

the text with the items in order

## Błędy

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## Przykład

```ahk
Sort("b`na`n", "N")
```

## Powiązane

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
