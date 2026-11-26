# `A_Index`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

正在运行的循环的轮次编号。第一个循环之前它是零；每个循环都保存它原有的值，并在结束时还回去。

## 语法

```ahk
A_Index 
```

## 参数

nothing; it is a variable, not a call

## 返回值

轮次编号，作为数字

## 错误

nothing to read; a script may write the name, and the loop goes on from the value that was written.

## 示例

```ahk
for item in [1, 2, 3]
    FileAppend A_Index, "*"
```
