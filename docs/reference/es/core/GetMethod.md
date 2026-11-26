# `GetMethod`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · method · `Object` · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

The method of an object, as a value that can be called.

## Sintaxis

```ahk
GetMethod 
```

## Parámetros

the object, the name of the method, and the number of parameters it would take

## Valor de retorno

the method, as a value of the class `Func`

## Errores

a method that is not there is refused with the type of the value and the name of the member.

## Ejemplo

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
