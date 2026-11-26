# `ControlSend`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Sends keystrokes to a control without activating the window.

## Sintaxe

```ahk
ControlSend Keys, Control, WinTitle
```

## Parâmetros

- **Keys** — The keystrokes.
- **Control** — The control (ClassNN).
- **WinTitle** — The window.

## Valor de retorno

Nothing.

## Exemplo

```ahk
ControlSend "hello", "Edit1", "ahk_class Notepad"
```

## Relacionados

[`Send`](Send.md), [`ControlSetText`](ControlSetText.md)
