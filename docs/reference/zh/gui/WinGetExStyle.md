# `WinGetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

窗口的扩展样式：Windows 的 `WS_EX_*` 位，它们说明窗口如何与其他窗口和屏幕相处。脚本自己的窗口的扩展样式是 256。

## 语法

```ahk
WinGetExStyle …
```

## 参数

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 返回值

扩展样式，作为数字

## 错误

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## 示例

```ahk
FileAppend WinGetExStyle("ZZZ_NO_WINDOW_ZZZ"), "*"
```
