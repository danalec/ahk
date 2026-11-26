# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · Janelas da GUI e automação de janelas

## Descrição

Creates a GUI window object.

## Sintaxe

```ahk
G := Gui(Options, Title)
```

## Parâmetros

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## Valor de retorno

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## Observações

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## Exemplo

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## Relacionados

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
