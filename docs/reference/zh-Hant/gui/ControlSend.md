# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Sends keystrokes to a control without activating the window.

## 語法

```ahk
ControlSend Keys, Control, WinTitle
```

## 參數

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## 傳回值

Nothing.

## 範例

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## 相關

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
