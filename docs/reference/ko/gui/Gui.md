# `Gui`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · GUI 창과 창 자동화

## 설명

Creates a GUI window object.

## 구문

```ahk
G := Gui(Options, Title)
```

## 매개변수

- **Options** — Styles: Resize, MinSize, AlwaysOnTop, ToolWindow, Owner.
- **Title** — The window title.

## 반환 값

A Gui object (Add/Show/Hide/Destroy/Submit, and the controls).

## 설명

On Windows the real window of user32; on the Linux box the GTK helper draws it.

## 예제

```ahk
g := Gui()
g.Add("Text", , "hello")
g.Show()
```

## 관련

[`Menu`](Menu.md), [`GuiControl`](GuiControl.md), [`MsgBox`](MsgBox.md)
