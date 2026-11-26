# `Process`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Машина, процессы и окружение

## Описание

Operates on a process (v1: Exist/Close/Wait/... ).

## Синтаксис

```ahk
Process Command, NameOrPID, Value
```

## Параметры

- **Command** — Exist/Close/Wait/WaitClose/Priority.
- **NameOrPID** — The target.
- **Value** — The parameter.

## Возвращаемое значение

The PID or 0.

## Пример

```ahk
Process "Exist", "notepad.exe"
```

## См. также

[`ProcessExist`](ProcessExist.md), [`ProcessClose`](ProcessClose.md)
