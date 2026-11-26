# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

The title of the active window, or of the window that matches the text that was given.

## Синтаксис

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## Параметры

- **WinTitle** — The criteria; omitted = last found.

## Возвращаемое значение

the title of the window

## Ошибки

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Пример

```ahk
title := WinGetTitle()
```

## См. также

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
