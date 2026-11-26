# `ControlGetFocus`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Reads the focused control.

## 構文

```ahk
S := ControlGetFocus(WinTitle)
```

## パラメータ

1..255.

## 戻り値

The ClassNN.

## 例

```ahk
ControlGetFocus("A")
```

## 関連

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
