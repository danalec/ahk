# `A_Now`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · variable · Tarihler ve saatler

## Açıklama

The local time of the machine, written as YYYYMMDDHH24MISS.

## Sözdizimi

```ahk
A_Now 
```

## Parametreler

nothing; it is a variable, not a call

## Dönüş değeri

the time, as a text of fourteen digits

## Hatalar

the read fails when the clock of the runtime does not know the date of the machine; the write is refused, because the name is a built-in variable that the dialect does not let a script write.

## Örnek

```ahk
FileAppend A_Now, "*"
```
