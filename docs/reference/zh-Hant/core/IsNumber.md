# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · function · 語言核心：文字、數學與模式

## 描述

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## 語法

```ahk
B := IsNumber(Value)
```

## 參數

- **Value** — Any value.

## 傳回值

1 when it is a number, 0 when it is not

## 錯誤

a second parameter is refused with "the code page of the text is not carried yet.".

## 範例

```ahk
IsNumber("42")
```

## 相關

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
