# `CallbackCreate`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings y entrada

## Descripción

Creates a callback address for a function (ks).

## Sintaxis

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## Parámetros

1..3.

## Valor de retorno

The address.

## Ejemplo

```ahk
p := CallbackCreate(Fn)
```

## Relacionados

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
