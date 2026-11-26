# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## Syntax

```ahk
ProcessClose NameOrPID
```

## Parameter

- **NameOrPID** — The name or PID.

## Rückgabewert

the pid of the process that was closed, 0 when it was not there or could not be closed

## Fehler

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Beispiel

```ahk
ProcessClose "firefox"
```

## Verwandt

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
