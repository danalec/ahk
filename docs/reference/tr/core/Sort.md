# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*KeySharp* · method · `Array` · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## Sözdizimi

```ahk
S := Sort(String, Options)
```

## Parametreler

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## Dönüş değeri

the text with the items in order

## Hatalar

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## Örnek

```ahk
Sort("b`na`n", "N")
```

## İlgili

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
