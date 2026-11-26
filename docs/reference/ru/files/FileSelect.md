# `FileSelect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Операции с файлами и каталогами

## Описание

Shows a file-selection dialog.

## Синтаксис

```ahk
S := FileSelect(Options, RootDir, Prompt, Filter)
```

## Параметры

0..4.

## Возвращаемое значение

The path(s) or empty.

## Пример

```ahk
p := FileSelect()
```

## См. также

[`DirSelect`](DirSelect.md), [`InputBox`](InputBox.md)
