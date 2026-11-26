# `CallbackCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · command · Kısayollar, tetikleme dizeleri ve giriş

## Açıklama

Creates a callback address for a function (ks).

## Sözdizimi

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## Parametreler

1..3.

## Dönüş değeri

The address.

## Örnek

```ahk
p := CallbackCreate(Fn)
```

## İlgili

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
