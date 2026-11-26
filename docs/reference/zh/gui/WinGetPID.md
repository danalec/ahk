# `WinGetPID`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

通过会话的无障碍总线给出窗口所属进程：总线登记应用，窗口是应用的孩子，总线给出它的 PID。

## 语法

```ahk
WinGetPID …
```

## 参数

WinTitle

## 返回值

该进程的 pid（整数）

## 错误

"Target window not found." when no window has that title; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."; the text of the window and the exclusions are refused with "the text of the window and the exclusions are not carried yet."

## 示例

```ahk
pid := WinGetPID("Calculator")
```
