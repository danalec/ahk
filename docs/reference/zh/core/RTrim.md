# `RTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 语言核心：文本、数学与模式

## 描述

去掉右端字符后的文本。

## 语法

```ahk
S := RTrim(String, OmitChars)
```

## 参数

- **String** — The text.
- **OmitChars** — Default space+tab.

## 返回值

去掉末尾那组字符之后的文本；调用方不指定时这组字符是空格和制表符

## 错误

none.

## 示例

```ahk
RTrim("  hello  ")
```

## 相关

[`Trim`](Trim.md), [`LTrim`](LTrim.md)
