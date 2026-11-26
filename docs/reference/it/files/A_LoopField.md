# `A_LoopField`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Operazioni su file e directory

## Descrizione

The field the parse loop is on: the piece of the text between two delimiters.

## Sintassi

```ahk
A_LoopField 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the field, as a text; the empty text before the first parse loop

## Errori

nothing to read; the write is refused.

## Esempio

```ahk
Loop Parse "a,b,c", ","
    FileAppend A_LoopField, "*"
```
