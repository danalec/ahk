# `ControlFindItem`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Окна интерфейса и автоматизация окон

## Описание

Finds an item by text.

## Синтаксис

```ahk
N := ControlFindItem(Text, Control, WinTitle)
```

## Параметры

2..6.

## Возвращаемое значение

The index or 0.

## Пример

```ahk
ControlFindItem("x", "ListBox1")
```

## См. также

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
