# `Sort`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*KeySharp* · method · `Array` · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The items of a text in order. The second parameter is the options, read letter by letter, and the third is a callback that compares two items.

## Sintaxis

```ahk
S := Sort(String, Options)
```

## Parámetros

- **String** — The lines.
- **Options** — N (numeric), R (reverse), D delim, C case, etc.

## Valor de retorno

the text with the items in order

## Errores

the first parameter must be a text; an array is refused; a third parameter that cannot be called fails with "This value of type ... has no method named Call".

## Ejemplo

```ahk
Sort("b`na`n", "N")
```

## Relacionados

[`StrSplit`](StrSplit.md), [`Array`](Array.md)
