# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Creates a GUI window object.

## Синтаксис

```ahk
G := Gui(Options, Title)
```

## Параметры

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## Возвращаемое значение

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## Замечания

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## Пример

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## См. также

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
