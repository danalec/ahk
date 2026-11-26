# `TrayTip`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## Sintaxis

```ahk
TrayTip Text, Title, Options
```

## Parámetros

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## Valor de retorno

the empty text

## Errores

the desktop reports the failure of the notification.

## Ejemplo

```ahk
TrayTip "done", "the script"
```

## Relacionados

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
