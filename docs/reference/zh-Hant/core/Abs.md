# `Abs`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

The distance from zero. An integer stays an integer.

## 語法

```ahk
N := Abs(Number)
```

## 參數

- **Number** — Integer or float.

## 傳回值

the number without its sign

## 錯誤

a value that is not a number is refused.

## 備註

Scalar; no SIMD gain (docs/simd.md).

## 範例

```ahk
Abs(-3)
```

## 相關

[`Mod`](Mod.md), [`Round`](Round.md), [`Min`](Min.md), [`Max`](Max.md)
