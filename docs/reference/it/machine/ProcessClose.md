# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · La macchina, i processi e l'ambiente

## Descrizione

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## Sintassi

```ahk
ProcessClose NameOrPID
```

## Parametri

- **NameOrPID** — The name or PID.

## Valore di ritorno

the pid of the process that was closed, 0 when it was not there or could not be closed

## Errori

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Esempio

```ahk
ProcessClose "firefox"
```

## Correlati

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
