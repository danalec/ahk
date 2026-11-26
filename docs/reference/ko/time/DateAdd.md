# `DateAdd`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · command · 날짜와 시간

## 설명

Answers a moment plus an amount of time; the date comes first and the unit comes last, and the unit is read by its first letter.

## 구문

```ahk
DateAdd …
```

## 매개변수

DateTime, Number, Unit

## 반환 값

the new date, as the text YYYYMMDDHH24MISS

## 오류

"Parameter #3 of DateAdd is invalid." when the unit is not one of d, h, m and s; "Parameter #1 of DateAdd is invalid." when the date is not one that v2 can read; "Parameter #2 of DateAdd requires a Number, but received a String."

## 예제

```ahk
next := DateAdd(20240101, 1, "d")
```
