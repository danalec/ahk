# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## 語法

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## 參數

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## 傳回值

nothing

## 錯誤

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## 範例

```ahk
WinClose "Untitled"
```

## 相關

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
