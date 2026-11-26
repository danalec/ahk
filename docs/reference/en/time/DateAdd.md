# `DateAdd`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · Dates and times

## Description

Answers a moment plus an amount of time; the date comes first and the unit comes last, and the unit is read by its first letter.

## Syntax

```ahk
DateAdd …
```

## Parameters

DateTime, Number, Unit

## Return value

the new date, as the text YYYYMMDDHH24MISS

## Errors

"Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

## Example

```ahk
next := DateAdd(20240101, 1, "d")
```
