# `SendMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Окна интерфейса и автоматизация окон

## Описание

Sends a message to a window and waits.

## Синтаксис

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## Параметры

1..9.

## Возвращаемое значение

The result.

## Пример

```ahk
SendMessage(0x100, 0, 0)
```

## См. также

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
