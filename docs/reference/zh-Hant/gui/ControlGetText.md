# `ControlGetText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Reads the text of a control.

## 語法

```ahk
T := ControlGetText(Control, WinTitle)
```

## 參數

- **Control** — ClassNN.
- **WinTitle** — The window.

## 傳回值

The text.

## 範例

```ahk
t := ControlGetText("Edit1", "ahk_class Notepad")
```

## 相關

[`ControlSetText`](ControlSetText.md), [`WinGetText`](WinGetText.md)
