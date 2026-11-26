# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Sends keystrokes to a control without activating the window.

## 構文

```ahk
ControlSend Keys, Control, WinTitle
```

## パラメータ

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## 戻り値

Nothing.

## 例

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## 関連

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
