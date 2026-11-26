# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · method · `Func` · 言語のオブジェクト: Array、Map、Func

## 説明

Binds arguments to a function, answering a BoundFunc.

## 構文

```ahk
F := Func.Bind(Args...)
```

## パラメータ

?.

## 戻り値

A BoundFunc.

## 例

```ahk
f := Func.Bind(1)
```

## 関連

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
