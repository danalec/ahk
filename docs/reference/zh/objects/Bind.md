# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Func` · 语言的对象：Array、Map 与 Func

## 描述

Binds arguments to a function, answering a BoundFunc.

## 语法

```ahk
F := Func.Bind(Args...)
```

## 参数

?.

## 返回值

A BoundFunc.

## 示例

```ahk
f := Func.Bind(1)
```

## 相关

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
