# `ProcessGetName`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · La máquina, los procesos y el entorno

## Descripción

Answers the name of the executable of a process; the name is the file name of /proc/<pid>/exe, and the name the kernel gives is the fallback.

## Sintaxis

```ahk
ProcessGetName …
```

## Parámetros

[ProcessNameOrPID]

## Valor de retorno

the name of the executable, as text

## Errores

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Ejemplo

```ahk
name := ProcessGetName(1234)
```
