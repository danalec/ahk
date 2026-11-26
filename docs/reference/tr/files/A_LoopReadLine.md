# `A_LoopReadLine`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Dosya ve dizin işlemleri

## Açıklama

The line of the file the read loop is on.

## Sözdizimi

```ahk
A_LoopReadLine 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the line, as a text; the empty text before the first read loop

## Hatalar

nothing to read; the write is refused.

## Örnek

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```
