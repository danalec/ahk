# `A_LoopField`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Dosya ve dizin işlemleri

## Açıklama

The field the parse loop is on: the piece of the text between two delimiters.

## Sözdizimi

```ahk
A_LoopField 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the field, as a text; the empty text before the first parse loop

## Hatalar

nothing to read; the write is refused.

## Örnek

```ahk
Loop Parse "a,b,c", ","
    FileAppend A_LoopField, "*"
```
