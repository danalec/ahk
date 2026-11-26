# `A_LoopFileName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Operacje na plikach i katalogach

## Opis

The name of the file the file loop is on, without its folder.

## Składnia

```ahk
A_LoopFileName 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the name of the file, as a text; the empty text before the first file loop

## Błędy

nothing to read; the write is refused.

## Przykład

```ahk
Loop Files "*.txt"
    FileAppend A_LoopFileName, "*"
```
