# `ProcessSetPriority`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Машина, процессы и окружение

## Описание

Sets the priority of a process.

## Синтаксис

```ahk
ProcessSetPriority Priority, NameOrPID
```

## Параметры

- **Priority** — L/B/N/A/H/R or a number.
- **NameOrPID** — The target.

## Возвращаемое значение

Nothing.

## Пример

```ahk
ProcessSetPriority "H", pid
```

## См. также

[`Process`](Process.md)
