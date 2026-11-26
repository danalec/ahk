# `IsSet`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*KeySharp* · function · El núcleo del lenguaje: texto, matemáticas y patrones

## Descripción

Whether the variable of the caller has a value. The tree carries it besides `IsSetRef`, and it asks for the variable and not for its value.

## Sintaxis

```ahk
IsSet 
```

## Parámetros

the variable, not its value

## Valor de retorno

1 when the variable has a value, 0 when it has none

## Errores

an argument that is not a variable is refused with "IsSet requires a variable.".

## Ejemplo

```ahk
x := 1
FileAppend IsSet(x), "*"
```
