# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · command · 解析器與腳本執行

## 描述

Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.

## 語法

```ahk
SetTimer Function, Period, Priority
```

## 參數

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## 傳回值

the integer 1

## 錯誤

more than two parameters are refused with "the priority of the timer is not carried yet."

## 備註

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## 範例

```ahk
SetTimer("Tick", 1000)
```

## 相關

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
