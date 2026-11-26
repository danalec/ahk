# `DefineProp`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · method · `Object` · Los objetos del lenguaje: Array, Map y Func

## Descripción

Defines a property with a getter/setter or value.

## Sintaxis

```ahk
Obj.DefineProp(Name, Desc)
```

## Parámetros

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Valor de retorno

Nothing.

## Ejemplo

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## Relacionados

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
