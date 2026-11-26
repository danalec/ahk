# `Random`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

A number between the two the caller names. Without arguments it is a number between zero and one; with one it is a whole number below it; with two it is a whole number between them, both ends counted.

## Sintassi

```ahk
N := Random(Min, Max)
```

## Parametri

- **Min/Max** — The range; omitted = float 0..1.
- **Seed** — With a single arg, reseeds.

## Valore di ritorno

the number

## Errori

none.

## Esempio

```ahk
Random(1, 6)
```

## Correlati

[`Sort`](Sort.md)
