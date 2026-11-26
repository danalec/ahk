# `ControlFindItem`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · GUIウィンドウとウィンドウの自動化

## 説明

Finds an item by text.

## 構文

```ahk
N := ControlFindItem(Text, Control, WinTitle)
```

## パラメータ

2..6.

## 戻り値

The index or 0.

## 例

```ahk
ControlFindItem("x", "ListBox1")
```

## 関連

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
