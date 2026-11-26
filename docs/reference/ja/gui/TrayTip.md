# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## 構文

```ahk
TrayTip Text, Title, Options
```

## パラメータ

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## 戻り値

the empty text

## エラー

the desktop reports the failure of the notification.

## 例

```ahk
TrayTip "done", "the script"
```

## 関連

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
