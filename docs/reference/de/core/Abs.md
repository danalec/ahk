# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · function · Der Kern der Sprache: Text, Mathematik und Muster

## Beschreibung

The distance from zero. An integer stays an integer.

## Syntax

```ahk
N := Abs(Number)
```

## Parameter

- **Number** — Integer or float.

## Rückgabewert

the number without its sign

## Fehler

a value that is not a number is refused.

## Bemerkungen

Scalar; no SIMD gain (docs/simd.md).

## Beispiel

```ahk
Abs(-3)
```

## Verwandt

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
