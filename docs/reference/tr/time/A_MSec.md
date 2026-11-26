# `A_MSec`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Tarihler ve saatler

## Açıklama

The milliseconds of the local time of the machine, in three digits. The number of v2 carries no millisecond, and this name does.

## Sözdizimi

```ahk
A_MSec 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the milliseconds, as a text of three digits

## Hatalar

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Örnek

```ahk
FileAppend A_MSec, "*"
```
