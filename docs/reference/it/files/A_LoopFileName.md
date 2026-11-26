# `A_LoopFileName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Operazioni su file e directory

## Descrizione

The name of the file the file loop is on, without its folder.

## Sintassi

```ahk
A_LoopFileName 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the name of the file, as a text; the empty text before the first file loop

## Errori

nothing to read; the write is refused.

## Esempio

```ahk
Loop Files "*.txt"
    FileAppend A_LoopFileName, "*"
```
