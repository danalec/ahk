# `ControlGetText`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Reads the text of a control.

## Syntax

```ahk
T := ControlGetText(Control, WinTitle)
```

## Parameters

- **Control** — ClassNN.
- **WinTitle** — The window.

## Return value

The text.

## Example

```ahk
t := ControlGetText("Edit1", "ahk_class Notepad")
```

## Related

[`ControlSetText`](ControlSetText.md), [`WinGetText`](WinGetText.md)
