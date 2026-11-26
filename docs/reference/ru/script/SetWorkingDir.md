# `SetWorkingDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Разбор и выполнение скрипта

## Описание

Sets the working directory of the script. A name that is not absolute is a folder under the working directory, the way the dialect says it.

## Синтаксис

```ahk
SetWorkingDir …
```

## Параметры

Directory

## Возвращаемое значение

the integer 1

## Ошибки

an empty name is refused with "the name of the directory is missing.", and a folder that cannot be entered fails with the reason of the desktop.

## Пример

```ahk
SetWorkingDir "."
```
