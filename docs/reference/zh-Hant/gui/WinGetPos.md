# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Where the window is and how big it is, as the compositor reports it.

## 語法

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## 參數

- **Output*** — Variables.
- **WinTitle** — The criteria.

## 傳回值

nothing; the place and the size are written into the variables of the caller

## 錯誤

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 範例

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## 相關

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
