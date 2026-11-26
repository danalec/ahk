# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Rdzeń języka: tekst, matematyka i wzorce

## Opis

The distance from zero. An integer stays an integer.

## Składnia

```ahk
N := Abs(Number)
```

## Parametry

- **Number** — Integer or float.

## Wartość zwracana

the number without its sign

## Błędy

a value that is not a number is refused.

## Uwagi

Scalar; no SIMD gain (docs/simd.md).

## Przykład

```ahk
Abs(-3)
```

## Powiązane

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
