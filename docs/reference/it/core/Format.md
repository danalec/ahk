# `Format`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

A pattern with the values in it. The pattern is the one of the reference dialect: {1} is the first argument, a placeholder with no number takes the next one, and {{ and }} are one brace of their own.

## Sintassi

```ahk
Text := Format(FormatStr, Values...)
```

## Parametri

- **FormatStr** — With {1}, {2:0.00}, {:#x} placeholders.
- **Values** — The values to insert.

## Valore di ritorno

the text of the pattern with the values in it

## Errori

a placeholder that names an argument that is not there, or whose spec is not one the dialect reads, is left exactly as it was written; a value of a kind the spec does not want is refused.

## Osservazioni

Supports width, precision, hex, and alignment.

## Esempio

```ahk
Format("{1} + {2} = {3}", 1, 2, 3)
```

## Correlati

[`StrReplace`](StrReplace.md), [`String`](String.md)
