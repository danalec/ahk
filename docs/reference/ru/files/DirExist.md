# `DirExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Операции с файлами и каталогами

## Описание

Answers the attributes of the first folder that the name or the pattern matches.

## Синтаксис

```ahk
A := DirExist(Path)
```

## Параметры

- **Path** — A path or wildcard.

## Возвращаемое значение

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## Ошибки

nothing fails.

## Пример

```ahk
x := DirExist("out")
```

## См. также

[`FileExist`](FileExist.md)
