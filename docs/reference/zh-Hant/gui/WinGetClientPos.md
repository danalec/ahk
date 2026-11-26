# `WinGetClientPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · GUI 視窗與視窗自動化

## 描述

The same place, of the part of the window the program draws in. A Wayland client does not draw its own frame -- the compositor does -- so the geometry the compositor reports is the window the user sees, and it is the answer of both commands.

## 語法

```ahk
WinGetClientPos …
```

## 參數

[WindowTitle] [, WinText, &X, &Y, &Width, &Height, ExcludeTitle, ExcludeText]

## 傳回值

nothing; the place and the size are written into the variables of the caller

## 錯誤

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## 範例

```ahk
WinGetClientPos "Untitled", , &x, &y, &w, &h
```
