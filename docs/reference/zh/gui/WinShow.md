# `WinShow`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

在 `WinHide` 之后，把脚本自己的窗口重新放回屏幕上。

## 语法

```ahk
WinShow …
```

## 参数

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 返回值

无返回值

## 错误

a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.

## 示例

```ahk
WinShow "Untitled"
```
