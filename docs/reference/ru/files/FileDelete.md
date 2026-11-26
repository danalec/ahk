# `FileDelete`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Операции с файлами и каталогами

## Описание

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Синтаксис

```ahk
FileDelete Pattern
```

## Параметры

- **Pattern** — A path or wildcard.

## Возвращаемое значение

nothing

## Ошибки

OSError when the file cannot be taken away.

## Пример

```ahk
FileDelete "out.txt"
```

## См. также

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
