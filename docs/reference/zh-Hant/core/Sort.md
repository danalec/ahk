# `Sort`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*KeySharp* · method · `Array` · 語言核心：文字、數學與模式

## 描述

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## 語法

```ahk
S := Sort(String, Options)
```

## 參數

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## 傳回值

the text with the items in order

## 錯誤

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## 範例

```ahk
Sort("b`na`n", "N")
```

## 相關

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
