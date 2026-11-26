# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Der Parser und die Ausführung des Skripts

## Beschreibung

Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.

## Syntax

```ahk
SetTimer Function, Period, Priority
```

## Parameter

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## Rückgabewert

the integer 1

## Fehler

more than two parameters are refused with "the priority of the timer is not carried yet."

## Bemerkungen

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## Beispiel

```ahk
SetTimer("Tick", 1000)
```

## Verwandt

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
