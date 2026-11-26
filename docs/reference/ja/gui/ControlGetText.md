# `ControlGetText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Reads the text of a control.

## 構文

```ahk
T := ControlGetText(Control, WinTitle)
```

## パラメータ

- **Control** — ClassNN.
- **WinTitle** — The window.

## 戻り値

The text.

## 例

```ahk
t := ControlGetText("Edit1", "ahk_class Notepad")
```

## 関連

[`ControlSetText`](ControlSetText.md), [`WinGetText`](WinGetText.md)
