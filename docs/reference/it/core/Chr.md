# `Chr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

The character with a code point.

## Sintassi

```ahk
S := Chr(Number)
```

## Parametri

- **Number** — The code point.

## Valore di ritorno

the text of that character

## Errori

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Esempio

```ahk
Chr(65)
```

## Correlati

[`Ord`](Ord.md), [`Asc`](Asc.md)
