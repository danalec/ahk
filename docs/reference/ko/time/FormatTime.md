# `FormatTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · command · 날짜와 시간

## 설명

A moment written the way a format says. The date comes first and the format comes last; a text that carries no date at all is the date of the machine.

## 구문

```ahk
S := FormatTime(YYYYMMDDHH24MISS, Format)
```

## 매개변수

- **Time** — A timestamp; omitted = now.
- **Format** — The picture (yyyy-MM-dd etc.).

## 반환 값

the date written the way the format says

## 오류

a date the dialect cannot read is an invalid parameter.

## 예제

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

## 관련

[`DateAdd`](DateAdd.md), [`DateDiff`](DateDiff.md), [`A_Now`](A_Now.md)
