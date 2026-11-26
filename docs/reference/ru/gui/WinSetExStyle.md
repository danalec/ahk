# `WinSetExStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Окна интерфейса и автоматизация окон

## Описание

Writes the extended style of a window. The style is absolute, the way the style of a window is: the text may carry a `+`, a `-` or a `^` before the number, and the runtime computes the new extended style out of the one that is there.

## Синтаксис

```ahk
WinSetExStyle …
```

## Параметры

Style, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Возвращаемое значение

the empty text

## Ошибки

an empty text refuses with 'Parameter #1 of WinSetExStyle is invalid.', and 'empty string' is in the `Extra`; a window that is not there refuses with 'Target window not found.'.

## Пример

```ahk
WinSetExStyle("+0x8", "ZZZ_NO_WINDOW_ZZZ")
```
