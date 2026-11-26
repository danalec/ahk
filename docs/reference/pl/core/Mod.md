# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## Składnia

```ahk
R := Mod(Dividend, Divisor)
```

## Parametry

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Wartość zwracana

the remainder

## Błędy

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Uwagi

A zero divisor throws ZeroDivisionError.

## Przykład

```ahk
Mod(7, 3)
```

## Powiązane

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
