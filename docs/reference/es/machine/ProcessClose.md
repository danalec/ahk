# `ProcessClose`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · La máquina, los procesos y el entorno

## Descripción

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## Sintaxis

```ahk
ProcessClose NameOrPID
```

## Parámetros

- **NameOrPID** — The name or PID.

## Valor de retorno

the pid of the process that was closed, 0 when it was not there or could not be closed

## Errores

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Ejemplo

```ahk
ProcessClose "firefox"
```

## Relacionados

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
