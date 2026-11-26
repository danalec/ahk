# `ScriptHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · variable · GUI 窗口与窗口自动化

## 描述

脚本那个隐藏窗口的句柄，即 v2 给它的名字。这个窗口就是拥有本次会话消息的那个，数字随运行而变。

## 语法

```ahk
ScriptHwnd 
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
