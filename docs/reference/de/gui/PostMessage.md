# `PostMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Posts a message to a window.

## Syntax

```ahk
PostMessage Msg, wParam, lParam, Control, WinTitle
```

## Parameter

1..255.

## Rückgabewert

Nothing.

## Beispiel

```ahk
PostMessage 0x100, 0, 0
```

## Verwandt

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
