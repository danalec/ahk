# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## Syntaxe

```ahk
TrayTip Text, Title, Options
```

## Paramètres

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## Valeur de retour

the empty text

## Erreurs

the desktop reports the failure of the notification.

## Exemple

```ahk
TrayTip "done", "the script"
```

## Voir aussi

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
