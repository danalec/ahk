# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · COM 자동화 (ComObject)

## 설명

Binds an object to a method, answering a BoundFunc.

## 구문

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## 매개변수

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## 반환 값

A BoundFunc.

## 예제

```ahk
f := ObjBindMethod(o, "m", 1)
```

## 관련

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
