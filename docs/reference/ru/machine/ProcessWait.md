# `ProcessWait`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Машина, процессы и окружение

## Описание

Waits for a process to be there; the box checks every hundred milliseconds, the way the binary of v2 does.

## Синтаксис

```ahk
ProcessWait …
```

## Параметры

ProcessNameOrPID, [Timeout]

## Возвращаемое значение

the pid of the process when it appears, 0 at the timeout

## Ошибки

a call without a timeout waits for ever; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Пример

```ahk
ProcessWait "firefox", 10
```
