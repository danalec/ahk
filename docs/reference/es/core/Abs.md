# `Abs`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The distance from zero. An integer stays an integer.

## Sintaxis

```ahk
N := Abs(Number)
```

## Parámetros

- **Number** — Integer or float.

## Valor de retorno

the number without its sign

## Errores

a value that is not a number is refused.

## Observaciones

Scalar; no SIMD gain (docs/simd.md).

## Ejemplo

```ahk
Abs(-3)
```

## Relacionados

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
