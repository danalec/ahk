# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

The title of the active window, or of the window that matches the text that was given.

## 語法

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## 參數

- **WinTitle** — The criteria; omitted = last found.

## 傳回值

the title of the window

## 錯誤

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## 範例

```ahk
title := WinGetTitle()
```

## 相關

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
