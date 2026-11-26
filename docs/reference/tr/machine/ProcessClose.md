# `ProcessClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Makine, süreçler ve ortam

## Açıklama

Closes a process; the box sends SIGTERM, the polite signal that asks the process to end.

## Sözdizimi

```ahk
ProcessClose NameOrPID
```

## Parametreler

- **NameOrPID** — The name or PID.

## Dönüş değeri

the pid of the process that was closed, 0 when it was not there or could not be closed

## Hatalar

a process of another user is a process the script cannot open, and the answer is zero and not a failure.

## Örnek

```ahk
ProcessClose "firefox"
```

## İlgili

[`ProcessExist`](ProcessExist.md), [`WinKill`](WinKill.md)
