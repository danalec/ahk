# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.

## Sözdizimi

```ahk
SetTimer Function, Period, Priority
```

## Parametreler

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## Dönüş değeri

the integer 1

## Hatalar

more than two parameters are refused with "the priority of the timer is not carried yet."

## Notlar

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## Örnek

```ahk
SetTimer("Tick", 1000)
```

## İlgili

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
