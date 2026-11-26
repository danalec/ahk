# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · L'analyseur et l'exécution du script

## Description

Registers a function of the script that runs between the statements. The runtime is one thread, so the timer is not a thread of its own: it runs between the statements and shares the budget of the script.

## Syntaxe

```ahk
SetTimer Function, Period, Priority
```

## Paramètres

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## Valeur de retour

the integer 1

## Erreurs

more than two parameters are refused with "the priority of the timer is not carried yet."

## Remarques

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## Exemple

```ahk
SetTimer("Tick", 1000)
```

## Voir aussi

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
