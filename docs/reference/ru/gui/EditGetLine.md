# `EditGetLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Окна интерфейса и автоматизация окон

## Описание

Reads a line of an edit.

## Синтаксис

```ahk
S := EditGetLine(N, Control, WinTitle)
```

## Параметры

2..6.

## Возвращаемое значение

The line.

## Пример

```ahk
EditGetLine(1, "Edit1")
```

## См. также

[`Gui`](Gui.md), [`ControlGetText`](ControlGetText.md)
