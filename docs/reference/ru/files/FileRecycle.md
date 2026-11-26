# `FileRecycle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

Sends a file, a folder, or the files that a pattern matches, to the trash of the machine. The box uses the trash of the XDG specification, and it is the home directory of the user that holds it.

## Синтаксис

```ahk
FileRecycle …
```

## Параметры

FileName

## Возвращаемое значение

nothing

## Ошибки

Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.

## Пример

```ahk
FileRecycle "out.txt"
```
