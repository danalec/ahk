# `StringGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Operazioni su file e directory

## Descrizione

The position of a substring (v1, 0-based).

## Sintassi

```ahk
StringGetPos Out, Haystack, Needle
```

## Parametri

3..255.

## Valore di ritorno

Nothing; 0-based pos or -1.

## Esempio

```ahk
StringGetPos p, s, "x"
```

## Correlati

[`InStr`](InStr.md)
