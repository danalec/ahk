# `ControlFindItem`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Finds an item by text.

## Syntax

```ahk
N := ControlFindItem(Text, Control, WinTitle)
```

## Parameter

2..6.

## Rückgabewert

The index or 0.

## Beispiel

```ahk
ControlFindItem("x", "ListBox1")
```

## Verwandt

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
