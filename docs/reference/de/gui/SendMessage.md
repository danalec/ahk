# `SendMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Sends a message to a window and waits.

## Syntax

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## Parameter

1..9.

## Rückgabewert

The result.

## Beispiel

```ahk
SendMessage(0x100, 0, 0)
```

## Verwandt

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
