# `A_ThisFunc`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

正在运行的函数的名称。

## 语法

```ahk
A_ThisFunc 
```

## 参数

nothing; it is a variable, not a call

## 返回值

名称，作为文本；没有函数运行时是空文本

## 错误

nothing to read; the write is refused.

## 示例

```ahk
FileAppend A_ThisFunc, "*"
```
