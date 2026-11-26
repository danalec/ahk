# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Sends keystrokes to a control without activating the window.

## Syntax

```ahk
ControlSend Keys, Control, WinTitle
```

## Parameter

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Rückgabewert

Nothing.

## Beispiel

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## Verwandt

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
