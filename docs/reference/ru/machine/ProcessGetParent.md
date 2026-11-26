# `ProcessGetParent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Машина, процессы и окружение

## Описание

The parent PID of a process.

## Синтаксис

```ahk
N := ProcessGetParent(NameOrPID)
```

## Параметры

- **NameOrPID** — The target.

## Возвращаемое значение

The parent PID.

## Пример

```ahk
pp := ProcessGetParent(pid)
```

## См. также

[`ProcessExist`](ProcessExist.md)
