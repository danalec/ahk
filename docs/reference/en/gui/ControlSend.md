# `ControlSend`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Sends keystrokes to a control without activating the window.

## Syntax

```ahk
ControlSend Keys, Control, WinTitle
```

## Parameters

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Return value

Nothing.

## Example

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## Related

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
