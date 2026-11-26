# `PostMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Posts a message to a window.

## Синтаксис

```ahk
PostMessage Msg, wParam, lParam, Control, WinTitle
```

## Параметры

1..255.

## Возвращаемое значение

Nothing.

## Пример

```ahk
PostMessage 0x100, 0, 0
```

## См. также

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
