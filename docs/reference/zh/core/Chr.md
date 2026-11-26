# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

具有某个码位的字符。

## 语法

```ahk
S := Chr(Number)
```

## 参数

- **Number** — The code point.

## 返回值

该字符的文本

## 错误

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## 示例

```ahk
Chr(65)
```

## 相关

[`Ord`](Ord.md), [`Asc`](Asc.md)
