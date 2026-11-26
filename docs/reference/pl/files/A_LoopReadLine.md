# `A_LoopReadLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Operacje na plikach i katalogach

## Opis

The line of the file the read loop is on.

## Składnia

```ahk
A_LoopReadLine 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the line, as a text; the empty text before the first read loop

## Błędy

nothing to read; the write is refused.

## Przykład

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
