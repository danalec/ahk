# `A_Sec`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · variable · Date e ore

## Descrizione

The second of the local time of the machine, in two digits.

## Sintassi

```ahk
A_Sec 
```

## Parametri

nothing; it is a variable, not a call

## Valore di ritorno

the second, as a text of two digits

## Errori

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Esempio

```ahk
FileAppend A_Sec, "*"
```
