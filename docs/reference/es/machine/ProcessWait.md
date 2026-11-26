# `ProcessWait`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · La máquina, los procesos y el entorno

## Descripción

Waits for a process to be there; the box checks every hundred milliseconds, the way the binary of v2 does.

## Sintaxis

```ahk
ProcessWait …
```

## Parámetros

ProcessNameOrPID, [Timeout]

## Valor de retorno

the pid of the process when it appears, 0 at the timeout

## Errores

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Ejemplo

```ahk
ProcessWait "firefox", 10
```
