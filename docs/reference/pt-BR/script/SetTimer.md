# `SetTimer`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · O analisador e a execução do script

## Descrição

Registra uma função do script que roda entre as instruções. O runtime tem uma só thread, então o timer não é uma thread própria: roda entre as instruções e divide o orçamento do script.

## Sintaxe

```ahk
SetTimer Function, Period, Priority
```

## Parâmetros

- **Function** — A function object, name, or label; omitted to delete.
- **Period** — Milliseconds; negative runs once; 0 deletes.
- **Priority** — The thread priority of the timer.

## Valor de retorno

o inteiro 1

## Erros

more than two parameters are refused with "the priority of the timer is not carried yet."

## Observações

The runtime runs timers between statements; a timer never interrupts a statement mid-way on this tree.

## Exemplo

```ahk
SetTimer("Tick", 1000)
```

## Relacionados

[`Sleep`](Sleep.md), [`Critical`](Critical.md), [`Thread`](Thread.md)
