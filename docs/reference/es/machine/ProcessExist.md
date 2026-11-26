# `ProcessExist`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · La máquina, los procesos y el entorno

## Descripción

Answers whether a process is there, and with which pid; the processes of the machine are read out of /proc.

## Sintaxis

```ahk
PID := ProcessExist(NameOrPID)
```

## Parámetros

- **NameOrPID** — The name or a PID.

## Valor de retorno

the pid of the process, or 0 when there is none

## Errores

a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none"; a process that is not there is zero and not a failure.

## Ejemplo

```ahk
if ProcessExist("firefox")
```

## Relacionados

[`ProcessClose`](ProcessClose.md), [`ProcessWait`](ProcessWait.md), [`Run`](Run.md)
