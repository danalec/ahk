# `StrLen`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Sintassi

```ahk
Count := StrLen(String)
```

## Parametri

- **String** — The text.

## Valore di ritorno

the number of code units

## Errori

none; a value that is not text is read as its text.

## Osservazioni

Astral characters count as one each on this tree.

## Esempio

```ahk
StrLen("hello")
```

## Correlati

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
