# `A_DD`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · variable · Operaciones de archivos y directorios

## Descripción

The day of the month of the local time, in two digits.

## Sintaxis

```ahk
A_DD 
```

## Parámetros

nothing; it is a variable, not a call

## Valor de retorno

the day, as a text of two digits

## Errores

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Ejemplo

```ahk
FileAppend A_DD, "*"
```
