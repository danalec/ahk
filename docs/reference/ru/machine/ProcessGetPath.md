# `ProcessGetPath`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Машина, процессы и окружение

## Описание

Answers the whole path of the executable of a process; the path is the target of the link /proc/<pid>/exe, and the " (deleted)" the kernel adds is cut.

## Синтаксис

```ahk
ProcessGetPath …
```

## Параметры

[ProcessNameOrPID]

## Возвращаемое значение

the path of the executable, as text

## Ошибки

"Target process not found." when the process is not there; a machine without /proc answers "the processes of this machine are read out of /proc, and this machine has none".

## Пример

```ahk
path := ProcessGetPath(1234)
```
