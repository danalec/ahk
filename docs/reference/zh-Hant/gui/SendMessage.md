# `SendMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · GUI 視窗與視窗自動化

## 描述

Sends a message to a window and waits.

## 語法

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## 參數

1..9.

## 傳回值

The result.

## 範例

```ahk
SendMessage(0x100, 0, 0)
```

## 相關

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
