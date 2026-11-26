# `DirCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Операции с файлами и каталогами

## Описание

Makes a directory, with the folders that are missing in the way.

## Синтаксис

```ahk
DirCreate Path
```

## Параметры

- **Path** — The directory.

## Возвращаемое значение

nothing

## Ошибки

OSError when the directory cannot be made.

## Пример

```ahk
DirCreate "out"
```

## См. также

[`DirDelete`](DirDelete.md), [`DirCopy`](DirCopy.md)
