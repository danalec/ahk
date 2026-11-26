# `Struct`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · class · 언어의 객체: Array, Map, Func

## 설명

A view of memory at a place, of a type asked for (ks).

## 구문

```ahk
s := Struct(Ptr, Type)
```

## 매개변수

?.

## 반환 값

A Struct with At/Size.

## 예제

```ahk
s := Struct(p, "Int")
```

## 관련

[`Buffer`](Buffer.md), [`NumGet`](NumGet.md)
