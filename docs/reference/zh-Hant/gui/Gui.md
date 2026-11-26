# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · GUI 視窗與視窗自動化

## 描述

Creates a GUI window object.

## 語法

```ahk
G := Gui(Options, Title)
```

## 參數

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## 傳回值

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## 備註

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## 範例

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## 相關

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
