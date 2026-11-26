# `A_MSec`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Daty i godziny

## Opis

The milliseconds of the local time of the machine, in three digits. The number of v2 carries no millisecond, and this name does.

## Składnia

```ahk
A_MSec 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the milliseconds, as a text of three digits

## Błędy

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Przykład

```ahk
FileAppend A_MSec, "*"
```
