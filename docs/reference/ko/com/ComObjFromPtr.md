# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · COM 자동화 (ComObject)

## 설명

Wraps a COM interface pointer as an object.

## 구문

```ahk
Obj := ComObjFromPtr(Ptr)
```

## 매개변수

- **Ptr** — The interface pointer.

## 반환 값

The object.

## 예제

```ahk
o := ComObjFromPtr(p)
```

## 관련

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
