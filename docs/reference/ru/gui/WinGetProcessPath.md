# `WinGetProcessPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Окна интерфейса и автоматизация окон

## Описание

The executable path of a window's process.

## Синтаксис

```ahk
S := WinGetProcessPath(WinTitle)
```

## Параметры

0..4.

## Возвращаемое значение

The path.

## Пример

```ahk
WinGetProcessPath("A")
```

## См. также

[`WinActivate`](WinActivate.md), [`WinExist`](WinExist.md)
