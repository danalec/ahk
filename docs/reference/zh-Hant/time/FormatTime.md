# `FormatTime`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 日期與時間

## 描述

A moment written the way a format says. The date comes first and the format comes last; a text that carries no date at all is the date of the machine.

## 語法

```ahk
S := FormatTime(YYYYMMDDHH24MISS, Format)
```

## 參數

- **Time** — A timestamp; omitted = now.
- **Format** — The picture (yyyy-MM-dd etc.).

## 傳回值

the date written the way the format says

## 錯誤

a date the dialect cannot read is an invalid parameter.

## 範例

```ahk
FormatTime(A_Now, "yyyy-MM-dd")
```

## 相關

[`DateAdd`](DateAdd.md), [`DateDiff`](DateDiff.md), [`A_Now`](A_Now.md)
