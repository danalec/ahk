# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Okna GUI i automatyzacja okien

## Opis

Sends keystrokes to a control without activating the window.

## Składnia

```ahk
ControlSend Keys, Control, WinTitle
```

## Parametry

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Wartość zwracana

Nothing.

## Przykład

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## Powiązane

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
