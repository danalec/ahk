# `Process`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · La máquina, los procesos y el entorno

## Descripción

Operates on a process (v1: Exist/Close/Wait/... ).

## Sintaxis

```ahk
Process Command, NameOrPID, Value
```

## Parámetros

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Valor de retorno

The PID or 0.

## Ejemplo

```ahk
Process "Exist", "notepad.exe"
```

## Relacionados

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
