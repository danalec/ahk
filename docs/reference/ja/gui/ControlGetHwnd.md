# `ControlGetHwnd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

Reads the handle of a control.

## 構文

```ahk
H := ControlGetHwnd(Control, WinTitle)
```

## パラメータ

1..5.

## 戻り値

The handle.

## 例

```ahk
ControlGetHwnd("Edit1")
```

## 関連

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
