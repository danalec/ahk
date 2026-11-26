# `TrayTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Sends a notification to the desktop with a text and a title. The name is a Windows one -- a balloon in the notification area -- but the tree sends a desktop notification, which reaches the same user with the same words.

## 구문

```ahk
TrayTip Text, Title, Options
```

## 매개변수

- **Text** — The message.
- **Title** — The title.
- **Options** — Icon/mute.

## 반환 값

the empty text

## 오류

the desktop reports the failure of the notification.

## 예제

```ahk
TrayTip "done", "the script"
```

## 관련

[`ToolTip`](ToolTip.md), [`MsgBox`](MsgBox.md)
