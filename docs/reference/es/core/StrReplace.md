# `StrReplace`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The text with one part of it swapped for another. The case sense says whether the letters are taken as they are, and it is off when the caller does not say.

## Sintaxis

```ahk
New := StrReplace(Haystack, Search, Replace, CaseSense, OutputVarCount, Limit)
```

## Parámetros

- **Haystack** — The text.
- **Search** — What to find.
- **Replace** — What to put (default empty).
- **CaseSense** — On/Off.
- **OutputVarCount** — Receives the number of replacements.
- **Limit** — Max replacements (-1 all).

## Valor de retorno

the text with the replacements

## Errores

the count must be a variable reference; a needle that is empty changes nothing and the text comes back as it is; the logical case sense is an invalid parameter.

## Observaciones

An empty Search is refused (Parameter #2 invalid).

## Ejemplo

```ahk
StrReplace("a-b-c", "-", "+")
```

## Relacionados

[`InStr`](InStr.md), [`RegExReplace`](RegExReplace.md), [`StrSplit`](StrSplit.md)
