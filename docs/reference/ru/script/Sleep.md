# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · command · Разбор и выполнение скрипта

## Описание

Pauses the script for a number of milliseconds.

## Синтаксис

```ahk
Sleep Milliseconds
```

## Параметры

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## Возвращаемое значение

the number of milliseconds waited

## Ошибки

none; a wait longer than 60000 milliseconds is cut at that number.

## Замечания

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## Пример

```ahk
Sleep 500
```

## См. также

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
