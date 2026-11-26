# `WinGetProcessName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

给出窗口所属程序的名称；它沿着 WinGetPID 的链条走：窗口在无障碍总线上找到应用，总线给出进程，进程表给出名称。

## 语法

```ahk
WinGetProcessName …
```

## 参数

WinTitle

## 返回值

可执行文件的文件名（文本）

## 错误

"Target window not found." when no window has that title, "Target process not found." when the process is gone by then; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."

## 示例

```ahk
name := WinGetProcessName("Calculator")
```
