# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

关闭窗口。它关闭的是脚本自己的窗口：合成器的窗口列表没有为别的程序的窗口提供关闭请求。

## 语法

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## 参数

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## 返回值

无返回值

## 错误

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## 示例

```ahk
WinClose "Untitled"
```

## 相关

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
