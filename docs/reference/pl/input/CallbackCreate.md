# `CallbackCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · command · Skróty, ciągi wyzwalające i wprowadzanie

## Opis

Creates a callback address for a function (ks).

## Składnia

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## Parametry

1..3.

## Wartość zwracana

The address.

## Przykład

```ahk
p := CallbackCreate(Fn)
```

## Powiązane

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
