# `PostMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Posts a message to a window.

## 語法

```ahk
PostMessage Msg, wParam, lParam, Control, WinTitle
```

## 參數

1..255.

## 傳回值

Nothing.

## 範例

```ahk
PostMessage 0x100, 0, 0
```

## 相關

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
