# `FileCreateShortcut`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

## Синтаксис

```ahk
FileCreateShortcut …
```

## Параметры

Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]

## Возвращаемое значение

nothing

## Ошибки

Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

## Пример

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```
