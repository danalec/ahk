# `SysGet`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · La máquina, los procesos y el entorno

## Descripción

Answers a metric of the system; the numbers are the ones of GetSystemMetrics, and only the ones about the screen are carried.

## Sintaxis

```ahk
SysGet …
```

## Parámetros

Metric

## Valor de retorno

the metric as an integer

## Errores

a metric of the window manager of Windows is refused with "SysGet(n) is a metric of the window manager of Windows, and this desktop does not have one."; a compositor that published no monitor answers "the compositor published no monitor, so the metric n of SysGet is not known."; a number that is not a metric at all is zero.

## Ejemplo

```ahk
w := SysGet(0)
```
