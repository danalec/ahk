# `IsNumber`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## Sintassi

```ahk
B := IsNumber(Value)
```

## Parametri

- **Value** — Any value.

## Valore di ritorno

1 when it is a number, 0 when it is not

## Errori

a second parameter is refused with "the code page of the text is not carried yet.".

## Esempio

```ahk
IsNumber("42")
```

## Correlati

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
