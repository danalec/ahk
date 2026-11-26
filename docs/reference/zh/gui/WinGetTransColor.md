# `WinGetTransColor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · GUI 窗口与窗口自动化

## 描述

窗口让过去的颜色：分层窗口的颜色键，也就是不被绘制的那个颜色。颜色写作 "0xRRGGBB"，按红、绿、蓝的位的顺序。窗口不是分层的时回答空文本。

## 语法

```ahk
WinGetTransColor …
```

## 参数

WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## 返回值

颜色，格式为 "0xRRGGBB"；窗口不是分层时为空文本

## 错误

a window that is not there refuses with 'Target window not found.', and the title that was asked about is in the `Extra`; the text of the window and the two exclusions are not carried yet.

## 示例

```ahk
FileAppend WinGetTransColor("ZZZ_NO_WINDOW_ZZZ"), "*"
```
