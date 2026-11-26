# `A_YYYY`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · variable · 日期與時間

## 描述

The year of the local time of the machine, in four digits.

## 語法

```ahk
A_YYYY 
```

## 參數

nothing; it is a variable, not a call

## 傳回值

the year, as a text of four digits

## 錯誤

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## 範例

```ahk
FileAppend A_YYYY, "*"
```
