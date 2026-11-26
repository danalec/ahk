# `ExitApp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Разбор и выполнение скрипта

## Описание

Ends the script with a code. The code goes into the exit_code of the report.

## Синтаксис

```ahk
ExitApp ExitCode
```

## Параметры

- **ExitCode** — The code the process exits with (default 0).

## Возвращаемое значение

the code

## Ошибки

none.

## Замечания

OnExit callbacks run before the process ends.

## Пример

```ahk
ExitApp 3
```

## См. также

[`Exit`](Exit.md), [`Reload`](Reload.md), [`OnExit`](OnExit.md)
