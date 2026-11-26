# `DateAdd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · command · 日期與時間

## 描述

Answers a moment plus an amount of time; the date comes first and the unit comes last, and the unit is read by its first letter.

## 語法

```ahk
DateAdd …
```

## 參數

DateTime, Number, Unit

## 傳回值

the new date, as the text YYYYMMDDHH24MISS

## 錯誤

"Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

## 範例

```ahk
next := DateAdd(20240101, 1, "d")
```
