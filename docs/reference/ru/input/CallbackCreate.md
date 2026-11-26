# `CallbackCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · command · Горячие клавиши, горячие строки и ввод

## Описание

Creates a callback address for a function (ks).

## Синтаксис

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## Параметры

1..3.

## Возвращаемое значение

The address.

## Пример

```ahk
p := CallbackCreate(Fn)
```

## См. также

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
