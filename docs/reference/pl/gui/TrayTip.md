# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Okna GUI i automatyzacja okien

## Opis

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## Składnia

```ahk
TrayTip Text, Title, Options
```

## Parametry

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## Wartość zwracana

the empty text

## Błędy

the desktop reports the failure of the notification.

## Przykład

```ahk
TrayTip "done", "the script"
```

## Powiązane

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
