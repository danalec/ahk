# `ToolTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Shows a small window of text at a place of the screen. Called with no text it takes the window away.

## 構文

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## パラメータ

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## 戻り値

the number that identifies the window, or the empty text when the window is taken away

## エラー

the desktop reports the failure of the call.

## 例

```ahk
ToolTip "hello", 10, 10
```

## 関連

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
