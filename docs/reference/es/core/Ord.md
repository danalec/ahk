# `Ord`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The code point of the first character of a text.

## Sintaxis

```ahk
N := Ord(String)
```

## Parámetros

- **String** — The text.

## Valor de retorno

the code point, or zero when the text has no character

## Errores

none; a lone surrogate answers with the code unit of it.

## Ejemplo

```ahk
Ord("A")
```

## Relacionados

[`Chr`](Chr.md), [`Asc`](Asc.md)
