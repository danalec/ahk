# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 语言核心：文本、数学与模式

## 描述

一个值是不是数字：整数、浮点数，或是写着数字的文本。十六进制文本也算数字。

## 语法

```ahk
B := IsNumber(Value)
```

## 参数

- **Value** — Any value.

## 返回值

是数字时为 1，否则为 0

## 错误

a second parameter is refused with "the code page of the text is not carried yet.".

## 示例

```ahk
IsNumber("42")
```

## 相关

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
