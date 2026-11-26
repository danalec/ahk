# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

The number to the number of decimals the caller asks for. With a positive number of decimals the answer is a text of exactly that many digits after the point, zeros kept; with the number omitted, zero or negative it is a whole number.

## Sintassi

```ahk
N := Round(Number, N)
```

## Parametri

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## Valore di ritorno

a text with the decimals asked for, or a whole number

## Errori

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## Osservazioni

Half-away-from-zero.

## Esempio

```ahk
Round(2.567, 2)
```

## Correlati

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
