# `A_TickCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 机器、进程与环境

## 描述

运行时从一个任意起点开始计数的毫秒数。计数从不后退。

## 语法

```ahk
A_TickCount 
```

## 参数

nothing; it is a variable, not a call

## 返回值

毫秒数，作为数字

## 错误

the write is refused; the name is a built-in variable that the dialect does not let a script write.

## 示例

```ahk
FileAppend A_TickCount, "*"
```
