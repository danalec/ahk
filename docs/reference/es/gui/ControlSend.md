# `ControlSend`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Sends keystrokes to a control without activating the window.

## Sintaxis

```ahk
ControlSend Keys, Control, WinTitle
```

## Parámetros

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## Relacionados

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
