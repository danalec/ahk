# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

文本第一个字符的码位。

## 语法

```ahk
N := Ord(String)
```

## 参数

- **String** — The text.

## 返回值

码位；文本没有字符时为 0

## 错误

none; a lone surrogate answers with the code unit of it.

## 示例

```ahk
Ord("A")
```

## 相关

[`Chr`](Chr.md), [`Asc`](Asc.md)
