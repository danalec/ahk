# `WinSetTransColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

写入窗口让过去的颜色。颜色是 HTML 的十七种颜色之一的名称、一个十六进制数字，或者单词 "Off"，它把颜色键去掉。

## 语法

```ahk
WinSetTransColor …
```

## 参数

Color, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 返回值

空文本

## 错误

a text that is not a colour refuses with 'Parameter #1 of WinSetTransColor is invalid.', and the text is in the `Extra`; a window that is not there refuses with 'Target window not found.'.

## 示例

```ahk
WinSetTransColor("Red", "ZZZ_NO_WINDOW_ZZZ")
```
