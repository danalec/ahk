# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Maszyna, procesy i środowisko

## Opis

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## Składnia

```ahk
ProcessClose NameOrPID
```

## Parametry

- **NameOrPID** — The name or PID.

## Wartość zwracana

the pid of the process that was closed, 0 when it was not there or could not be closed

## Błędy

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Przykład

```ahk
ProcessClose "firefox"
```

## Powiązane

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
