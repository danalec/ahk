# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · method · `Object` · Dilin nesneleri: Array, Map ve Func

## Açıklama

Defines a property with a getter/setter or value.

## Sözdizimi

```ahk
Obj.DefineProp(Name, Desc)
```

## Parametreler

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Dönüş değeri

Nothing.

## Örnek

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## İlgili

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
