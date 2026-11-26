# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v1.1.37.02* · function · Ядро языка: текст, математика и шаблоны

## Описание

The number to the number of decimals the caller asks for. With a positive number of decimals the answer is a text of exactly that many digits after the point, zeros kept; with the number omitted, zero or negative it is a whole number.

## Синтаксис

```ahk
N := Round(Number, N)
```

## Параметры

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## Возвращаемое значение

a text with the decimals asked for, or a whole number

## Ошибки

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## Замечания

Half-away-from-zero.

## Пример

```ahk
Round(2.567, 2)
```

## См. также

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
