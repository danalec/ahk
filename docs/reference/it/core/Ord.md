# `Ord`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

The code point of the first character of a text.

## Sintassi

```ahk
N := Ord(String)
```

## Parametri

- **String** — The text.

## Valore di ritorno

the code point, or zero when the text has no character

## Errori

none; a lone surrogate answers with the code unit of it.

## Esempio

```ahk
Ord("A")
```

## Correlati

[`Chr`](Chr.md), [`Asc`](Asc.md)
