# `RTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · function · 語言核心：文字、數學與模式

## 描述

The text without the characters at its right end.

## 語法

```ahk
S := RTrim(String, OmitChars)
```

## 參數

- **String** — The text.
- **OmitChars** — Default space+tab.

## 傳回值

the text without the trailing characters of the set; the set is the space and the tab when the caller does not name one

## 錯誤

none.

## 範例

```ahk
RTrim("  hello  ")
```

## 相關

[`Trim`](Trim.md), [`LTrim`](LTrim.md)
