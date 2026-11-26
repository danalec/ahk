# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · variable · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## Składnia

```ahk
A_SendMode 
```

## Parametry

nothing; it is a variable, not a call

## Wartość zwracana

the name of the mode, as a text

## Błędy

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Przykład

```ahk
FileAppend A_SendMode, "*"
```
