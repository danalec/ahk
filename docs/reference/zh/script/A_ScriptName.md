# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

脚本的名称。运行器的脚本来自标准输入，没有文件，所以名称是一个星号。脚本可以写这个名称，写入的值就是它保留的值。

## 语法

```ahk
A_ScriptName 
```

## 参数

nothing; it is a variable, not a call

## 返回值

文本 "*"，或写入的文本

## 错误

nothing to read; the write takes any value.

## 示例

```ahk
FileAppend A_ScriptName, "*"
```
