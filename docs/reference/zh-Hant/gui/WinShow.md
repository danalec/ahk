# `WinShow`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Puts the window of the script back on the screen, after a `WinHide`.

## 語法

```ahk
WinShow …
```

## 參數

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## 傳回值

nothing

## 錯誤

a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.

## 範例

```ahk
WinShow "Untitled"
```
