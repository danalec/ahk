# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · method · `Object` · Die Objekte der Sprache: Array, Map und Func

## Beschreibung

Defines a property with a getter/setter or value.

## Syntax

```ahk
Obj.DefineProp(Name, Desc)
```

## Parameter

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Rückgabewert

Nothing.

## Beispiel

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## Verwandt

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
