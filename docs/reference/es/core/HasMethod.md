# `HasMethod`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · method · `Object` · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

Whether an object has a method.

## Sintaxis

```ahk
HasMethod 
```

## Parámetros

the object, the name of the method, and the number of parameters it would take

## Valor de retorno

1 when the object has such a method, 0 when it has not

## Errores

nothing; a value that is not an object answers zero, and the question with no name asks whether the value is a class.

## Ejemplo

```ahk
o := {a: 1}
FileAppend HasMethod(o, "a"), "*"
```
