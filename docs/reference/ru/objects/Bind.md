# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · method · `Func` · Объекты языка: Array, Map и Func

## Описание

Binds arguments to a function, answering a BoundFunc.

## Синтаксис

```ahk
F := Func.Bind(Args...)
```

## Параметры

?.

## Возвращаемое значение

A BoundFunc.

## Пример

```ahk
f := Func.Bind(1)
```

## См. также

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
