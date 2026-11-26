# `WinActivate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Brings a window to the front. It acts on the window of the script itself: the listing of the compositor publishes no way to raise a window of another program.

## 語法

```ahk
WinActivate WinTitle, WinText, ExcludeTitle, ExcludeText
```

## 參數

- **WinTitle** — Title/ahk_class/ahk_exe/ahk_id/ahk_pid.
- **WinText** — A text the window must contain.
- **Exclude*** — Criteria to skip.

## 傳回值

nothing

## 錯誤

a window that is not there answers 'Target window not found.'; a window of another program cannot be activated this way.

## 備註

On Wayland the compositor may refuse focus; the tree reports the reason.

## 範例

```ahk
WinActivate "Untitled"
```

## 相關

[`WinWait`](WinWait.md), [`WinActive`](WinActive.md), [`WinClose`](WinClose.md)
