# `Struct`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · class · 语言的对象：Array、Map 与 Func

## 描述

A view of memory at a place, of a type asked for (ks).

## 语法

```ahk
s := Struct(Ptr, Type)
```

## 参数

?.

## 返回值

A Struct with At/Size.

## 示例

```ahk
s := Struct(p, "Int")
```

## 相关

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
