# `StringGetPos`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · command · Operaciones de archivos y directorios

## Descripción

The position of a substring (v1, 0-based).

## Sintaxis

```ahk
StringGetPos Out, Haystack, Needle
```

## Parámetros

3..255.

## Valor de retorno

Nothing; 0-based pos or -1.

## Ejemplo

```ahk
StringGetPos p, s, "x"
```

## Relacionados

[`InStr`](InStr.md)
