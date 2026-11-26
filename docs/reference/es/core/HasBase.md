# `HasBase`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · method · `Object` · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

Whether the base of an object is another object.

## Sintaxis

```ahk
HasBase 
```

## Parámetros

the object, and the base

## Valor de retorno

1 when the base is that one, 0 when it is not

## Errores

a base that is not an object is refused before the question of the object is asked.

## Ejemplo

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
