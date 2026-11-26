# `RegExMatch`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

Where a pattern is in a text. The position counts from one and is zero when the pattern is not there.

## 語法

```ahk
RegExMatch …
```

## 參數

Text, Pattern, [&Output], [Start]

## 傳回值

the position of the match, or 0 when there is none

## 錯誤

the output must be a variable reference; the output holds an object of the class RegExMatchInfo, and a text that does not match leaves it holding the empty text.

## 範例

```ahk
RegExMatch("abc123", "\d+")
```
