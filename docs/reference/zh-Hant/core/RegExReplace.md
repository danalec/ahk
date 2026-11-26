# `RegExReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

The text with the matches of a pattern replaced. The replacement may name the groups of the pattern with $1 or \1, and $$ is a dollar of its own.

## 語法

```ahk
RegExReplace …
```

## 參數

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## 傳回值

the text with the replacements

## 錯誤

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## 範例

```ahk
RegExReplace("a1b2", "\d", "#")
```
