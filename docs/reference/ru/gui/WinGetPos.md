# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Where the window is and how big it is, as the compositor reports it.

## Синтаксис

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Параметры

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Возвращаемое значение

nothing; the place and the size are written into the variables of the caller

## Ошибки

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Пример

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## См. также

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
