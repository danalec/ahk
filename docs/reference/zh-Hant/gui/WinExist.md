# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · GUI 視窗與視窗自動化

## 描述

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## 語法

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## 參數

- **WinTitle** — The criteria.

## 傳回值

the identifier of the window that is there, zero when there is none

## 錯誤

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## 備註

Sets the last-found window for the Win* family.

## 範例

```ahk
if WinExist("Untitled")
```

## 相關

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
