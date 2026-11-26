# `DateDiff`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Fechas y horas

## Descripción

Answers the difference between two moments; the answer is the first date minus the second, truncated towards zero.

## Sintaxis

```ahk
DateDiff …
```

## Parámetros

DateTime1, DateTime2, Unit

## Valor de retorno

the difference, as an integer, in the unit that was asked for

## Errores

"Parameter #3 of DateDiff is invalid." when the unit is not one of d, h, m and s; a date that v2 cannot read is an invalid parameter.

## Ejemplo

```ahk
days := DateDiff(20240110, 20240101, "d")
```
