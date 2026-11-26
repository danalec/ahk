# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

Sends keystrokes to a control without activating the window.

## Sintassi

```ahk
ControlSend Keys, Control, WinTitle
```

## Parametri

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Valore di ritorno

Nothing.

## Esempio

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## Correlati

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
