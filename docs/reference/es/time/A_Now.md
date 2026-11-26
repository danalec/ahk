# `A_Now`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Fechas y horas

## Descripción

The local time of the machine, written as YYYYMMDDHH24MISS.

## Sintaxis

```ahk
A_Now 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the time, as a text of fourteen digits

## Errores

the read fails when the clock of the runtime does not know the date of the machine; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

## Ejemplo

```ahk
FileAppend A_Now, "*"
```
