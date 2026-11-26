# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · command · A máquina, os processos e o ambiente

## Descrição

Operates on a process (v1: Exist/Close/Wait/... ).

## Sintaxe

```ahk
Process Command, NameOrPID, Value
```

## Parâmetros

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Valor de retorno

The PID or 0.

## Exemplo

```ahk
Process "Exist", "notepad.exe"
```

## Relacionados

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
