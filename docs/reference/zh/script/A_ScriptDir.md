# `A_ScriptDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

运行开始的目录。运行器的脚本来自标准输入，本身没有文件，所以脚本的目录就是运行的目录。

## 语法

```ahk
A_ScriptDir 
```

## 参数

nothing; it is a variable, not a call

## 返回值

目录的路径，作为文本

## 错误

the read fails when the runtime does not keep the directory where the run started; the write is refused.

## 示例

```ahk
FileAppend A_ScriptDir, "*"
```
