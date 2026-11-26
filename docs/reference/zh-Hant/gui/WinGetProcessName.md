# `WinGetProcessName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · GUI 視窗與視窗自動化

## 描述

Answers the name of the program of a window; it rides the chain of WinGetPID -- the window names an application on the accessibility bus, the bus answers its process, and the process table answers the name.

## 語法

```ahk
WinGetProcessName …
```

## 參數

WinTitle

## 傳回值

the file name of the executable, as text

## 錯誤

"Target window not found." when no window has that title, "Target process not found." when the process is gone by then; an empty title is refused with "the window is found by its title, and an empty title is not carried yet."

## 範例

```ahk
name := WinGetProcessName("Calculator")
```
