# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## 語法

```ahk
Count := StrLen(String)
```

## 參數

- **String** — The text.

## 傳回值

the number of code units

## 錯誤

none; a value that is not text is read as its text.

## 備註

Astral characters count as one each on this tree.

## 範例

```ahk
StrLen("hello")
```

## 相關

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
