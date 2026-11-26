# `A_NowUTC`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · variable · Dates and times

## Description

The time of the machine in UTC, written as YYYYMMDDHH24MISS.

## Syntax

```ahk
A_NowUTC 
```

## Parameters

nothing; it is a variable, not a call

## Return value

the time, as a text of fourteen digits

## Errors

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Example

```ahk
FileAppend A_NowUTC, "*"
```
