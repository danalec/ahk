# `DateAdd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · command · 日付と時刻

## 説明

Answers a moment plus an amount of time; the date comes first and the unit comes last, and the unit is read by its first letter.

## 構文

```ahk
DateAdd …
```

## パラメータ

DateTime, Number, Unit

## 戻り値

the new date, as the text YYYYMMDDHH24MISS

## エラー

"Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

## 例

```ahk
next := DateAdd(20240101, 1, "d")
```
