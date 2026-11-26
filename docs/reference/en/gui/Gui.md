# `Gui`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · GUI windows and window automation

## Description

Creates a GUI window object.

## Syntax

```ahk
G := Gui(Options, Title)
```

## Parameters

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## Return value

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## Remarks

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## Example

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## Related

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
