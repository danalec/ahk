# `Object`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Los objetos del lenguaje: Array, Map y Func

## Descripción

The root of the chain of the classes: every object of the runtime is one. `Object()` makes an object with nothing in it, and the literal `{a: 1}` makes one with a property.

## Sintaxis

```ahk
Obj := Object(Props...)
```

## Parámetros

- **Props** — Alternating names and values.

## Valor de retorno

the object of the class `Object`

## Errores

nothing; a member that is not there is refused with the type of the value and the name of the member.

## Observaciones

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## Ejemplo

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## Relacionados

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
