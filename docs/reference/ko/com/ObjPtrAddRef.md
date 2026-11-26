# `ObjPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · COM 자동화 (ComObject)

## 설명

The pointer of an object, adding a reference.

## 구문

```ahk
P := ObjPtrAddRef(Obj)
```

## 매개변수

- **Obj** — The object.

## 반환 값

The pointer with a reference.

## 예제

```ahk
p := ObjPtrAddRef(o)
```

## 관련

[`ObjPtr`](ObjPtr.md), [`ObjRelease`](ObjRelease.md)
