# `RegExMatch`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

Where a pattern is in a text. The position counts from one and is zero when the pattern is not there.

## Sintaxis

```ahk
RegExMatch …
```

## Parámetros

Text, Pattern, [&Output], [Start]

## Valor de retorno

the position of the match, or 0 when there is none

## Errores

the output must be a variable reference; the output holds an object of the class RegExMatchInfo, and a text that does not match leaves it holding the empty text.

## Ejemplo

```ahk
RegExMatch("abc123", "\d+")
```
