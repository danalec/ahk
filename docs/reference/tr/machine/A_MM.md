# `A_MM`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Makine, süreçler ve ortam

## Açıklama

The month of the local time of the machine, in two digits.

## Sözdizimi

```ahk
A_MM 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the month, as a text of two digits

## Hatalar

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Örnek

```ahk
FileAppend A_MM, "*"
```
