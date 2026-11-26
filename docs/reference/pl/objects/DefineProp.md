# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · method · `Object` · Obiekty języka: Array, Map i Func

## Opis

Defines a property with a getter/setter or value.

## Składnia

```ahk
Obj.DefineProp(Name, Desc)
```

## Parametry

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Wartość zwracana

Nothing.

## Przykład

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## Powiązane

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
