# `Bind`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · method · `Func` · 언어의 객체: Array, Map, Func

## 설명

Binds arguments to a function, answering a BoundFunc.

## 구문

```ahk
F := Func.Bind(Args...)
```

## 매개변수

?.

## 반환 값

A BoundFunc.

## 예제

```ahk
f := Func.Bind(1)
```

## 관련

[`BoundFunc`](BoundFunc.md), [`ObjBindMethod`](ObjBindMethod.md)
