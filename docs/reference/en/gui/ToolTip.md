# `ToolTip`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Shows a small window of text at a place of the screen. Called with no text it takes the window away.

## Syntax

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## Parameters

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## Return value

the number that identifies the window, or the empty text when the window is taken away

## Errors

the desktop reports the failure of the call.

## Example

```ahk
ToolTip "hello", 10, 10
```

## Related

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
