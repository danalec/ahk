# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Il nucleo del linguaggio: testo, matematica e modelli

## Descrizione

The distance from zero. An integer stays an integer.

## Sintassi

```ahk
N := Abs(Number)
```

## Parametri

- **Number** — Integer or float.

## Valore di ritorno

the number without its sign

## Errori

a value that is not a number is refused.

## Osservazioni

Scalar; no SIMD gain (docs/simd.md).

## Esempio

```ahk
Abs(-3)
```

## Correlati

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
