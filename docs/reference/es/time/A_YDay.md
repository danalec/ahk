# `A_YDay`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Fechas y horas

## Descripción

The day of the year of the local time, 1 being the first of January.

## Sintaxis

```ahk
A_YDay 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the day of the year, as a text of a number

## Errores

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Ejemplo

```ahk
FileAppend A_YDay, "*"
```
