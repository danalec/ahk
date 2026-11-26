# `WinHide`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Окна интерфейса и автоматизация окон

## Описание

Takes the window of the script off the screen.

## Синтаксис

```ahk
WinHide …
```

## Параметры

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Возвращаемое значение

nothing

## Ошибки

a window that is not there answers 'Target window not found.'; a window of another program cannot be hidden this way.

## Пример

```ahk
WinHide "Untitled"
```
