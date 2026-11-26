# `Abs`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · The core of the language: text, math and patterns

## Description

The distance from zero. An integer stays an integer.

## Syntax

```ahk
N := Abs(Number)
```

## Parameters

- **Number** — Integer or float.

## Return value

the number without its sign

## Errors

a value that is not a number is refused.

## Remarks

Scalar; no SIMD gain (docs/simd.md).

## Example

```ahk
Abs(-3)
```

## Related

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
