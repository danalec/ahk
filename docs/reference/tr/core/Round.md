# `Round`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · Dilin çekirdeği: metin, matematik ve kalıplar

## Açıklama

The number to the number of decimals the caller asks for. With a positive number of decimals the answer is a text of exactly that many digits after the point, zeros kept; with the number omitted, zero or negative it is a whole number.

## Sözdizimi

```ahk
N := Round(Number, N)
```

## Parametreler

- **Number** — The value.
- **N** — Decimals; negative rounds tens/hundreds.

## Dönüş değeri

a text with the decimals asked for, or a whole number

## Hatalar

a text that does not hold a number is refused; more than 100000 decimals is an invalid parameter.

## Notlar

Half-away-from-zero.

## Örnek

```ahk
Round(2.567, 2)
```

## İlgili

[`Floor`](Floor.md), [`Ceil`](Ceil.md), [`Mod`](Mod.md)
