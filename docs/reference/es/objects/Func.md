# `Func`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Los objetos del lenguaje: Array, Map y Func

## Descripción

The class of a function: a function of the script, a method, a lambda or a built-in. The class is not made: `Func(...)` answers "Invalid base."; the class of a function value is `Func`.

## Sintaxis

```ahk
Func …
```

## Parámetros

nothing that makes it

## Valor de retorno

the class of a function value, as a text

## Errores

the call of the class is refused with "Invalid base.".

## Ejemplo

```ahk
FileAppend Abs.__Class, "*"
```
