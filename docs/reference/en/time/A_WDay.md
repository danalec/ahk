# `A_WDay`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Dates and times

## Description

The day of the week of the local time, 1 being the Sunday and 7 the Saturday.

## Syntax

```ahk
A_WDay 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the day of the week, as a text of a number

## Errors

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Example

```ahk
FileAppend A_WDay, "*"
```
