# `FileReadLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

Reads a line by number (v1).

## Синтаксис

```ahk
FileReadLine Out, Filename, LineNum
```

## Параметры

3..255.

## Возвращаемое значение

Nothing.

## Пример

```ahk
FileReadLine s, "a.txt", 2
```

## См. также

[`FileRead`](FileRead.md), [`LoopRead`](LoopRead.md)
