# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · GUI 窗口与窗口自动化

## 描述

Creates a GUI window object.

## 语法

```ahk
G := Gui(Options, Title)
```

## 参数

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## 返回值

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## 备注

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## 示例

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## 相关

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
