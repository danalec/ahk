# `ObjGetBase`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · Los objetos del lenguaje: Array, Map y Func

## Descripción

The base of an object: the prototype of its class.

## Sintaxis

```ahk
ObjGetBase …
```

## Parámetros

the object

## Valor de retorno

the object of the base

## Errores

a value that is not an object is refused with "Expected an object but got ...".

## Ejemplo

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
