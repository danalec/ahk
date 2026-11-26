# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## Sözdizimi

```ahk
TrayTip Text, Title, Options
```

## Parametreler

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## Dönüş değeri

the empty text

## Hatalar

the desktop reports the failure of the notification.

## Örnek

```ahk
TrayTip "done", "the script"
```

## İlgili

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
