# `ToolTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Shows a small window of text at a place of the screen. Called with no text it takes the window away.

## 語法

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## 參數

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## 傳回值

the number that identifies the window, or the empty text when the window is taken away

## 錯誤

the desktop reports the failure of the call.

## 範例

```ahk
ToolTip "hello", 10, 10
```

## 相關

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
