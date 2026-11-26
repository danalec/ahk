# `A_LoopReadLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Операции с файлами и каталогами

## Описание

The line of the file the read loop is on.

## Синтаксис

```ahk
A_LoopReadLine 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

the line, as a text; the empty text before the first read loop

## Ошибки

nothing to read; the write is refused.

## Пример

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
