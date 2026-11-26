# `A_YYYY`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Даты и время

## Описание

The year of the local time of the machine, in four digits.

## Синтаксис

```ahk
A_YYYY 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

the year, as a text of four digits

## Ошибки

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Пример

```ahk
FileAppend A_YYYY, "*"
```
