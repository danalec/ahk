# `Chr`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The character with a code point.

## Sintaxis

```ahk
S := Chr(Number)
```

## Parámetros

- **Number** — The code point.

## Valor de retorno

the text of that character

## Errores

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Ejemplo

```ahk
Chr(65)
```

## Relacionados

[`Ord`](Ord.md), [`Asc`](Asc.md)
