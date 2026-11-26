# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

The code point of the first character of a text.

## 語法

```ahk
N := Ord(String)
```

## 參數

- **String** — The text.

## 傳回值

the code point, or zero when the text has no character

## 錯誤

none; a lone surrogate answers with the code unit of it.

## 範例

```ahk
Ord("A")
```

## 相關

[`Chr`](Chr.md), [`Asc`](Asc.md)
