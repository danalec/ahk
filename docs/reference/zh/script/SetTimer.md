# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · command · 解析器与脚本执行

## 描述

注册一个在语句之间运行的脚本函数。这个运行时只有一个线程，因此定时器不是独立的线程：它在语句之间运行，与脚本共用预算。

## 语法

```ahk
SetTimer Function, Period, Priority
```

## 参数

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## 返回值

整数 1

## 错误

more than two parameters are refused with "the priority of the timer is not carried yet."

## 备注

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## 示例

```ahk
SetTimer("Tick", 1000)
```

## 相关

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
