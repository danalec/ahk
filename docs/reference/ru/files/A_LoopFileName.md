# `A_LoopFileName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Операции с файлами и каталогами

## Описание

The name of the file the file loop is on, without its folder.

## Синтаксис

```ahk
A_LoopFileName 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

the name of the file, as a text; the empty text before the first file loop

## Ошибки

nothing to read; the write is refused.

## Пример

```ahk
Loop Files "*.txt"
    FileAppend A_LoopFileName, "*"
```
