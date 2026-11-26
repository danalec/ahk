# `CallbackCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · command · 热键、热字符串与输入

## 描述

Creates a callback address for a function (ks).

## 语法

```ahk
P := CallbackCreate(Func, Options, ParamCount)
```

## 参数

1..3.

## 返回值

The address.

## 示例

```ahk
p := CallbackCreate(Fn)
```

## 相关

[`RegisterCallback`](RegisterCallback.md), [`CallbackFree`](CallbackFree.md), [`DllCall`](DllCall.md)
