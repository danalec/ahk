# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*KeySharp* · method · `Array` · 言語の中核: テキスト、数値、正規表現

## 説明

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## 構文

```ahk
S := Sort(String, Options)
```

## パラメータ

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## 戻り値

the text with the items in order

## エラー

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## 例

```ahk
Sort("b`na`n", "N")
```

## 関連

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
