# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · command · Parser i wykonanie skryptu

## Opis

Pauses the script for a number of milliseconds.

## Składnia

```ahk
Sleep Milliseconds
```

## Parametry

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## Wartość zwracana

the number of milliseconds waited

## Błędy

none; a wait longer than 60000 milliseconds is cut at that number.

## Uwagi

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## Przykład

```ahk
Sleep 500
```

## Powiązane

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
