# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · method · `Object` · Gli oggetti del linguaggio: Array, Map e Func

## Descrizione

Defines a property with a getter/setter or value.

## Sintassi

```ahk
Obj.DefineProp(Name, Desc)
```

## Parametri

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Valore di ritorno

Nothing.

## Esempio

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## Correlati

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
