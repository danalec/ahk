# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Sends keystrokes to a control without activating the window.

## Sözdizimi

```ahk
ControlSend Keys, Control, WinTitle
```

## Parametreler

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Dönüş değeri

Nothing.

## Örnek

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## İlgili

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
