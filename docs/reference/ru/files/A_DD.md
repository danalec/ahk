# `A_DD`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · variable · Операции с файлами и каталогами

## Описание

The day of the month of the local time, in two digits.

## Синтаксис

```ahk
A_DD 
```

## Параметры

nothing; it is a variable, not a call

## Возвращаемое значение

the day, as a text of two digits

## Ошибки

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Пример

```ahk
FileAppend A_DD, "*"
```
