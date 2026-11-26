# `Mod`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · The core of the language: text, math and patterns

## Description

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## Syntax

```ahk
R := Mod(Dividend, Divisor)
```

## Parameters

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Return value

the remainder

## Errors

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Remarks

A zero divisor throws ZeroDivisionError.

## Example

```ahk
Mod(7, 3)
```

## Related

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
