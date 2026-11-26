# `RegExReplace`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The text with the matches of a pattern replaced. The replacement may name the groups of the pattern with $1 or \1, and $$ is a dollar of its own.

## Sintaxis

```ahk
RegExReplace …
```

## Parámetros

Text, Pattern, Replacement, [&Count], [Limit], [Start]

## Valor de retorno

the text with the replacements

## Errores

the count must be a variable reference; a pattern that does not compile fails with the reason of the compiler.

## Ejemplo

```ahk
RegExReplace("a1b2", "\d", "#")
```
