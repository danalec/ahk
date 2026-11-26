# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · command · パーサとスクリプトの実行

## 説明

Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.

## 構文

```ahk
SetTimer Function, Period, Priority
```

## パラメータ

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## 戻り値

the integer 1

## エラー

more than two parameters are refused with "the priority of the timer is not carried yet."

## 備考

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## 例

```ahk
SetTimer("Tick", 1000)
```

## 関連

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
