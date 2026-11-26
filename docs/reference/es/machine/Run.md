# `Run`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · La máquina, los procesos y el entorno

## Descripción

Runs a command and waits for it. What the command wrote on its output goes into the output of the report.

## Sintaxis

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## Parámetros

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## Valor de retorno

the exit code of the command

## Errores

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Observaciones

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## Ejemplo

```ahk
Run "ls"
```

## Relacionados

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
