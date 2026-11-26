# `A_LoopField`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · variable · Datei- und Verzeichnisoperationen

## Beschreibung

The field the parse loop is on: the piece of the text between two delimiters.

## Syntax

```ahk
A_LoopField 
```

## Parameter

nothing; it is a variable, not a call

## Rückgabewert

the field, as a text; the empty text before the first parse loop

## Fehler

nothing to read; the write is refused.

## Beispiel

```ahk
Loop Parse "a,b,c", ","
    FileAppend A_LoopField, "*"
```
