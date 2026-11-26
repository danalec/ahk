# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · method · `Object` · Объекты языка: Array, Map и Func

## Описание

Iterates the own properties of an object.

## Синтаксис

```ahk
for name, value in Obj.OwnProps()
```

## Параметры

?.

## Возвращаемое значение

The enumerator.

## Пример

```ahk
for n, v in o.OwnProps()
```

## См. также

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
