# `Reload`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Разбор и выполнение скрипта

## Описание

Starts the script again in a process of its own: the old instance ends and the new one carries on with the same file and the same arguments. A script that comes from the standard input has no file, and the v2 does nothing then.

## Синтаксис

```ahk
Reload
```

## Параметры

none

## Возвращаемое значение

the empty text

## Ошибки

none; the name takes no parameter, and a script without a file is not refused -- the call simply does nothing.

## Замечания

The new instance replaces the old; single-instance rules apply.

## Пример

```ahk
Reload
```

## См. также

[`ExitApp`](ExitApp.md), [`#SingleInstance`](#SingleInstance.md)
