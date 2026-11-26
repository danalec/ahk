# `ProcessClose`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · The machine, the processes and the environment

## Description

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## Syntax

```ahk
ProcessClose NameOrPID
```

## Parameters

- **NameOrPID** — The name or PID.

## Return value

the pid of the process that was closed, 0 when it was not there or could not be closed

## Errors

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Example

```ahk
ProcessClose "firefox"
```

## Related

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
