# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · GUI 窗口与窗口自动化

## 描述

判断是否存在某个标题的窗口。窗口来自合成器，数字标识其中一个。

## 语法

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## 参数

- **WinTitle** — The criteria.

## 返回值

存在时返回该窗口的标识，不存在时为 0

## 错误

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## 备注

Sets the last-found window for the Win* family.

## 示例

```ahk
if WinExist("Untitled")
```

## 相关

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
