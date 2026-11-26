# `CallbackCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · command · Hotkeys, Hotstrings und Eingabe

## Beschreibung

Creates a callback address for a function (ks).

## Syntax

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## Parameter

1..3.

## Rückgabewert

The address.

## Beispiel

```ahk
p := CallbackCreate(Fn)
```

## Verwandt

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
