# `SysGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · Die Maschine, die Prozesse und die Umgebung

## Beschreibung

Answers a metric of the system; the numbers are the ones of GetSystemMetrics, and only the ones about the screen are carried.

## Syntax

```ahk
SysGet …
```

## Parameter

Metric

## Rückgabewert

the metric as an integer

## Fehler

a metric of the window manager of Windows is refused with "SysGet(n) is a metric of the window manager of Windows, and this desktop does not have one."; a compositor that published no monitor answers "the compositor published no monitor, so the metric n of SysGet is not known."; a number that is not a metric at all is zero.

## Beispiel

```ahk
w := SysGet(0)
```
