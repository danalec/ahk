# `Thread`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

运行线程的状态。命令是 "Priority"、"Interrupt" 或 "NoTimers"，单词不区分大小写："Priority" 取线程的优先级，"Interrupt" 取中断的毫秒数和行数，"NoTimers" 说明定时器是否可运行——这是 v2 的双重否定，"NoTimers" 为零时定时器可以运行。

## 语法

```ahk
Thread …
```

## 参数

Command, [Value1], [Value2]

## 返回值

空文本

## 错误

a command that is not one of the three refuses with "Parameter #1 of Thread is invalid.", and the word is in the `Extra`; a value that is not there leaves the setting as it was.

## 示例

```ahk
Thread "Priority", 5
```
