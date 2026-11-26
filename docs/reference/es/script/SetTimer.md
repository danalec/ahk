# `SetTimer`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · El analizador y la ejecución del script

## Descripción

Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.

## Sintaxis

```ahk
SetTimer Function, Period, Priority
```

## Parámetros

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## Valor de retorno

the integer 1

## Errores

more than two parameters are refused with "the priority of the timer is not carried yet."

## Observaciones

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## Ejemplo

```ahk
SetTimer("Tick", 1000)
```

## Relacionados

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
