# `SubStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

文本中两个位置之间的部分。位置从 1 开始，负位置从末尾倒数，-1 是最后一个字符。

## 语法

```ahk
Part := SubStr(String, Start, Length)
```

## 参数

- **String** — The text.
- **Start** — 1-based; negative counts from the end.
- **Length** — Omitted = to the end.

## 返回值

该部分的文本；负的长度会从末尾去掉相应数量的字符

## 错误

none; the text is cut in the code units of UTF-16, so the second unit of a surrogate pair is a text of its own.

## 备注

Counts characters, not bytes.

## 示例

```ahk
SubStr("hello", 2, 3)
```

## 相关

[`StrLen`](StrLen.md), [`InStr`](InStr.md), [`StrSplit`](StrSplit.md)
