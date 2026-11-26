# `Bind`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · method · `Func` · Los objetos del lenguaje: Array, Map y Func

## Descripción

Binds arguments to a function, answering a BoundFunc.

## Sintaxis

```ahk
F := Func.Bind(Args...)
```

## Parámetros

?.

## Valor de retorno

A BoundFunc.

## Ejemplo

```ahk
f := Func.Bind(1)
```

## Relacionados

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
