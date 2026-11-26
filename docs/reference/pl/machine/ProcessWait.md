# `ProcessWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Maszyna, procesy i środowisko

## Opis

Waits for a process to be there; the box checks every hundred milliseconds, the way the binary of v2 does.

## Składnia

```ahk
ProcessWait …
```

## Parametry

ProcessNameOrPID, [Timeout]

## Wartość zwracana

the pid of the process when it appears, 0 at the timeout

## Błędy

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Przykład

```ahk
ProcessWait "firefox", 10
```
