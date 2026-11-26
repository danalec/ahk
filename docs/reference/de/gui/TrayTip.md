# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## Syntax

```ahk
TrayTip Text, Title, Options
```

## Parameter

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## Rückgabewert

the empty text

## Fehler

the desktop reports the failure of the notification.

## Beispiel

```ahk
TrayTip "done", "the script"
```

## Verwandt

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
