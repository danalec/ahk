# `IsNumber`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## Sintaxis

```ahk
B := IsNumber(Value)
```

## Parámetros

- **Value** — Any value.

## Valor de retorno

1 when it is a number, 0 when it is not

## Errores

a second parameter is refused with "the code page of the text is not carried yet.".

## Ejemplo

```ahk
IsNumber("42")
```

## Relacionados

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
