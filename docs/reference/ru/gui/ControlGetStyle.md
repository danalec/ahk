# `ControlGetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Окна интерфейса и автоматизация окон

## Описание

The style of a control of a window: the number of the bits of Windows, the way the style of a window is. A control is a window of its own, and it is found by its name, its number or its class.

## Синтаксис

```ahk
ControlGetStyle …
```

## Параметры

Control, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Возвращаемое значение

the style, as a number

## Ошибки

a window that is not there refuses with 'Target window not found.', and a control that is not there with 'Target control not found.'; the text of the window and the two exclusions are not carried yet.

## Пример

```ahk
FileAppend ControlGetStyle("Edit1", "ZZZ_NO_WINDOW_ZZZ"), "*"
```
