# `ProcessExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Makine, süreçler ve ortam

## Açıklama

Answers whether a process is there, and with which pid; the processes of the machine are read out of /proc.

## Sözdizimi

```ahk
PID := ProcessExist(NameOrPID)
```

## Parametreler

- **NameOrPID** — The name or a PID.

## Dönüş değeri

the pid of the process, or 0 when there is none

## Hatalar

a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none"; a process that is not there is zero and not a failure.

## Örnek

```ahk
if ProcessExist("firefox")
```

## İlgili

[`ProcessClose`](ProcessClose.md), [`ProcessWait`](ProcessWait.md), [`Run`](Run.md)
