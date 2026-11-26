# `Gui`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Ventanas de la GUI y automatización de ventanas

## Descripción

Creates a GUI window object.

## Sintaxis

```ahk
G := Gui(Options, Title)
```

## Parámetros

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## Valor de retorno

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## Observaciones

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## Ejemplo

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## Relacionados

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
