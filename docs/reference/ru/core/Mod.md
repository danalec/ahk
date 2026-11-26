# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The remainder of a division. The sign of the remainder is the sign of the dividend, and the answer is a whole number when both of the arguments are.

## Синтаксис

```ahk
R := Mod(Dividend, Divisor)
```

## Параметры

- **Dividend** — The number.
- **Divisor** — Non-zero.

## Возвращаемое значение

the remainder

## Ошибки

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## Замечания

A zero divisor throws ZeroDivisionError.

## Пример

```ahk
Mod(7, 3)
```

## См. также

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
