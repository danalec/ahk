# `A_LoopReadLine`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Operaciones de archivos y directorios

## Descripción

The line of the file the read loop is on.

## Sintaxis

```ahk
A_LoopReadLine 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the line, as a text; the empty text before the first read loop

## Errores

nothing to read; the write is refused.

## Ejemplo

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
