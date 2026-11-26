# `Struct`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*KeySharp* · class · 語言的物件：Array、Map 與 Func

## 描述

A view of memory at a place, of a type asked for (ks).

## 語法

```ahk
s := Struct(Ptr, Type)
```

## 參數

?.

## 傳回值

A Struct with At/Size.

## 範例

```ahk
s := Struct(p, "Int")
```

## 相關

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
