# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 解析器與腳本執行

## 描述

Pauses the script for a number of milliseconds.

## 語法

```ahk
Sleep Milliseconds
```

## 參數

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## 傳回值

the number of milliseconds waited

## 錯誤

none; a wait longer than 60000 milliseconds is cut at that number.

## 備註

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## 範例

```ahk
Sleep 500
```

## 相關

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
