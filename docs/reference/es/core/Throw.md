# `Throw`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*KeySharp* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

A statement that raises a value. The value is usually an error object, and it may be any value.

## Sintaxis

```ahk
Throw 
```

## Parámetros

Value

## Valor de retorno

nothing; the statement leaves the flow of the script

## Errores

none; the value is caught by the catch that names its class.

## Ejemplo

```ahk
throw ValueError("boom")
```
