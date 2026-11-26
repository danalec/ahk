# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

The character with a code point.

## 語法

```ahk
S := Chr(Number)
```

## 參數

- **Number** — The code point.

## 傳回值

the text of that character

## 錯誤

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## 範例

```ahk
Chr(65)
```

## 相關

[`Ord`](Ord.md), [`Asc`](Asc.md)
