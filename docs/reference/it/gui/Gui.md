# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

Creates a GUI window object.

## Sintassi

```ahk
G := Gui(Options, Title)
```

## Parametri

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## Valore di ritorno

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## Osservazioni

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## Esempio

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## Correlati

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
