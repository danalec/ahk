# `VerCompare`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

比较两个版本字符串。两个版本的数字逐个比较，缺少的组件按 0 处理。带预发布后缀的版本比不带它的同版本更旧。

## 语法

```ahk
VerCompare …
```

## 参数

Version1, Version2

## 返回值

两个版本的顺序，-1、0 或 1；第二个版本带运算符（<、<=、>、>= 或 =）时返回该关系是否成立

## 错误

none.

## 示例

```ahk
VerCompare("1.2.0", "1.10.0")
```
