# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Окна интерфейса и автоматизация окон

## Описание

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## Синтаксис

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Параметры

- **WinTitle** — The criteria.

## Возвращаемое значение

the identifier of the window that is there, zero when there is none

## Ошибки

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Замечания

Sets the last-found window for the Win* family.

## Пример

```ahk
if WinExist("Untitled")
```

## См. также

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
