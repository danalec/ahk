# `WinActivate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

把窗口置于最前。它作用于脚本自己的窗口：合成器的窗口列表没有提供把别的程序的窗口提到前面的办法。

## 语法

```ahk
WinActivate WinTitle, WinText, ExcludeTitle, ExcludeText
```

## 参数

- **WinTitle** — Title/ahk_class/ahk_exe/ahk_id/ahk_pid.
- **WinText** — A text the window must contain.
- **Exclude*** — Criteria to skip.

## 返回值

无返回值

## 错误

a window that is not there answers 'Target window not found.'; a window of another program cannot be activated this way.

## 备注

On Wayland the compositor may refuse focus; the tree reports the reason.

## 示例

```ahk
WinActivate "Untitled"
```

## 相关

[`WinWait`](WinWait.md), [`WinActive`](WinActive.md), [`WinClose`](WinClose.md)
