# `Process`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · The machine, the processes and the environment

## Description

Operates on a process (v1: Exist/Close/Wait/... ).

## Syntax

```ahk
Process Command, NameOrPID, Value
```

## Parameters

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Return value

The PID or 0.

## Example

```ahk
Process "Exist", "notepad.exe"
```

## Related

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
