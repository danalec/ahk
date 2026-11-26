# `StrSplit`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

文本按分隔符切开的片段。分隔符是字面量：文本是一个分隔符，文本数组是一组分隔符，其中最先出现的那一个获胜。

## 语法

```ahk
Array := StrSplit(String, Delimiters, OmitChars, MaxParts)
```

## 参数

- **String** — The text.
- **Delimiters** — A string of delimiter chars or an array.
- **OmitChars** — Chars to trim from each part.
- **MaxParts** — Limit the number of parts.

## 返回值

片段的数组；要省略的字符会从每个片段的首尾切掉

## 错误

the delimiters must be a text or an array of texts; an empty array is refused with "Parameter #2 of StrSplit is invalid.".

## 备注

Empty delimiters split into characters.

## 示例

```ahk
StrSplit("a,b,c", ",")
```

## 相关

[`StrReplace`](StrReplace.md), [`Array`](Array.md), [`LoopParse`](LoopParse.md)
