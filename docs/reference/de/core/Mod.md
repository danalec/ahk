# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## Syntax

```ahk
R := Mod(Dividend, Divisor)
```

## Parameter

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Rückgabewert

the remainder

## Fehler

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Bemerkungen

A zero divisor throws ZeroDivisionError.

## Beispiel

```ahk
Mod(7, 3)
```

## Verwandt

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
