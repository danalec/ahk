# `A_LoopFileName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 文件与目录操作

## 描述

文件循环所在的文件的名称，不含文件夹。

## 语法

```ahk
A_LoopFileName 
```

## 参数

nothing; it is a variable, not a call

## 返回值

文件名，作为文本；文件循环之前是空文本

## 错误

nothing to read; the write is refused.

## 示例

```ahk
Loop Files "*.txt"
    FileAppend A_LoopFileName, "*"
```
