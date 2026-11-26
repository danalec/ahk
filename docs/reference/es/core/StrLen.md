# `StrLen`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Sintaxis

```ahk
Count := StrLen(String)
```

## Parámetros

- **String** — The text.

## Valor de retorno

the number of code units

## Errores

none; a value that is not text is read as its text.

## Observaciones

Astral characters count as one each on this tree.

## Ejemplo

```ahk
StrLen("hello")
```

## Relacionados

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
