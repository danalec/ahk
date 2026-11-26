# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · method · `Object` · Объекты языка: Array, Map и Func

## Описание

Defines a property with a getter/setter or value.

## Синтаксис

```ahk
Obj.DefineProp(Name, Desc)
```

## Параметры

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## Возвращаемое значение

Nothing.

## Пример

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## См. также

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
