# `CallbackCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · command · Hotkeys, hotstrings e entrada

## Descrição

Creates a callback address for a function (ks).

## Sintaxe

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## Parâmetros

1..3.

## Valor de retorno

The address.

## Exemplo

```ahk
p := CallbackCreate(Fn)
```

## Relacionados

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
