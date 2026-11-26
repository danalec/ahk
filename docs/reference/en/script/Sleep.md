# `Sleep`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · Parser and script execution

## Description

Pauses the script for a number of milliseconds.

## Syntax

```ahk
Sleep Milliseconds
```

## Parameters

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## Return value

the number of milliseconds waited

## Errors

none; a wait longer than 60000 milliseconds is cut at that number.

## Remarks

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## Example

```ahk
Sleep 500
```

## Related

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
