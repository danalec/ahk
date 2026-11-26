# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · method · `Func` · Dilin nesneleri: Array, Map ve Func

## Açıklama

Binds arguments to a function, answering a BoundFunc.

## Sözdizimi

```ahk
F := Func.Bind(Args...)
```

## Parametreler

?.

## Dönüş değeri

A BoundFunc.

## Örnek

```ahk
f := Func.Bind(1)
```

## İlgili

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
