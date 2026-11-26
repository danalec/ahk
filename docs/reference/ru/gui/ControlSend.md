# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Sends keystrokes to a control without activating the window.

## Синтаксис

```ahk
ControlSend Keys, Control, WinTitle
```

## Параметры

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Возвращаемое значение

Nothing.

## Пример

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## См. также

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
