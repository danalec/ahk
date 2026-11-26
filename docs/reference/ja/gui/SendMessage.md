# `SendMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

Sends a message to a window and waits.

## 構文

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## パラメータ

1..9.

## 戻り値

The result.

## 例

```ahk
SendMessage(0x100, 0, 0)
```

## 関連

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
