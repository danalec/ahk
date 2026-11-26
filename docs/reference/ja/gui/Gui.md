# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · GUIウィンドウとウィンドウの自動化

## 説明

Creates a GUI window object.

## 構文

```ahk
G := Gui(Options, Title)
```

## パラメータ

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## 戻り値

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## 備考

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## 例

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## 関連

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
