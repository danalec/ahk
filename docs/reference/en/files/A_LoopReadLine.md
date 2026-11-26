# `A_LoopReadLine`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · File and directory operations

## Description

The line of the file the read loop is on.

## Syntax

```ahk
A_LoopReadLine 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the line, as a text; the empty text before the first read loop

## Errors

nothing to read; the write is refused.

## Example

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
