# `Format`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

把数值填入模式的文本。模式是参考方言的那一套：{1} 是第一个参数，没有编号的占位符取下一个参数，{{ 和 }} 各代表一个花括号。

## 语法

```ahk
Text := Format(FormatStr, Values...)
```

## 参数

- **FormatStr** — With {1}, {2:0.00}, {:#x} placeholders.
- **Values** — The values to insert.

## 返回值

填入数值之后的模式文本

## 错误

a placeholder that names an argument that is not there, or whose spec is not one the dialect reads, is left exactly as it was written; a value of a kind the spec does not want is refused.

## 备注

Supports width, precision, hex, and alignment.

## 示例

```ahk
Format("{1} + {2} = {3}", 1, 2, 3)
```

## 相关

[`StrReplace`](StrReplace.md), [`String`](String.md)
