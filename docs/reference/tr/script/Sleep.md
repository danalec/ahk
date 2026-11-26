# `Sleep`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · command · Ayrıştırıcı ve betiğin çalıştırılması

## Açıklama

Pauses the script for a number of milliseconds.

## Sözdizimi

```ahk
Sleep Milliseconds
```

## Parametreler

- **Milliseconds** — 0..60000; a larger value is clamped to 60000, a negative one sleeps 0.

## Dönüş değeri

the number of milliseconds waited

## Hatalar

none; a wait longer than 60000 milliseconds is cut at that number.

## Notlar

The pause is clamped, not refused. During the sleep the timers of the script may still run (the runtime checks them between statements).

## Örnek

```ahk
Sleep 500
```

## İlgili

[`SetTimer`](SetTimer.md), [`Critical`](Critical.md)
