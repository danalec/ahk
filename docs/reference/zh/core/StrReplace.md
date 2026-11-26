# `StrReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

把文本中的一段换成另一段得到的文本。大小写敏感方式说明是否区分大小写，调用方不说时为不区分。

## 语法

```ahk
New := StrReplace(Haystack, Search, Replace, CaseSense, OutputVarCount, Limit)
```

## 参数

- **Haystack** — The text.
- **Search** — What to find.
- **Replace** — What to put (default empty).
- **CaseSense** — On/Off.
- **OutputVarCount** — Receives the number of replacements.
- **Limit** — Max replacements (-1 all).

## 返回值

替换之后的文本

## 错误

the count must be a variable reference; a needle that is empty changes nothing and the text comes back as it is; the logical case sense is an invalid parameter.

## 备注

An empty Search is refused (Parameter #2 invalid).

## 示例

```ahk
StrReplace("a-b-c", "-", "+")
```

## 相关

[`InStr`](InStr.md), [`RegExReplace`](RegExReplace.md), [`StrSplit`](StrSplit.md)
