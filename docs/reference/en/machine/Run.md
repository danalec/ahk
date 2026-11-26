# `Run`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · The machine, the processes and the environment

## Description

Runs a command and waits for it. What the command wrote on its output goes into the output of the report.

## Syntax

```ahk
Run Target, WorkingDir, Options, OutputVarPID
```

## Parameters

- **Target** — A program, document, URL or verb.
- **WorkingDir** — The working directory of the child.
- **Options** — Min/Max/Hide/UseErrorLevel.
- **OutputVarPID** — A variable that receives the process id.

## Return value

the exit code of the command

## Errors

more than two parameters are refused with "the options of the window and the pid of the process are not carried yet."

## Remarks

On Linux it is sh -c; on Windows ShellExecute/CreateProcess. The child outlives the script unless killed.

## Example

```ahk
Run "ls"
```

## Related

[`RunWait`](RunWait.md), [`Process`](Process.md), [`WinWait`](WinWait.md)
