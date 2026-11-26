# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## Sintassi

```ahk
R := Mod(Dividend, Divisor)
```

## Parametri

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Valore di ritorno

the remainder

## Errori

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Osservazioni

A zero divisor throws ZeroDivisionError.

## Esempio

```ahk
Mod(7, 3)
```

## Correlati

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
