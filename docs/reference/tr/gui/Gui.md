# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · GUI pencereleri ve pencere otomasyonu

## Açıklama

Creates a GUI window object.

## Sözdizimi

```ahk
G := Gui(Options, Title)
```

## Parametreler

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## Dönüş değeri

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## Notlar

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## Örnek

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## İlgili

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
