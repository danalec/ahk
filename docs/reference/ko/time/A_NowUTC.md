# `A_NowUTC`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · variable · 날짜와 시간

## 설명

The time of the machine in UTC, written as YYYYMMDDHH24MISS.

## 구문

```ahk
A_NowUTC 
```

## 매개변수

nothing; it is a variable, not a call

## 반환 값

the time, as a text of fourteen digits

## 오류

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## 예제

```ahk
FileAppend A_NowUTC, "*"
```
