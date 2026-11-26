# `A_ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · variable · 解析器与脚本执行

## 描述

脚本那个隐藏窗口的句柄：拥有本次会话消息的窗口。这个数字随运行而变——v2.0.31 有一次运行回答 6557770，另一次回答 59116662。

## 语法

```ahk
A_ScriptHwnd 
```

## 参数

nothing; it is a variable, not a call

## 返回值

窗口的句柄，作为数字

## 错误

the read fails with the reason of the desktop when the desktop has no window of the script; the write is refused.

## 示例

```ahk
FileAppend A_ScriptHwnd, "*"
```
