# `LTrim`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

The text without the characters at its left end.

## Sintassi

```ahk
S := LTrim(String, OmitChars)
```

## Parametri

- **String** — The text.
- **OmitChars** — Default space+tab.

## Valore di ritorno

the text without the leading characters of the set; the set is the space and the tab when the caller does not name one

## Errori

none.

## Esempio

```ahk
LTrim("  hello  ")
```

## Correlati

[`Trim`](Trim.md), [`RTrim`](RTrim.md)
