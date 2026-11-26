# `ObjRelease`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · COM 자동화 (ComObject)

## 설명

Releases a reference to an object pointer.

## 구문

```ahk
ObjRelease Ptr
```

## 매개변수

- **Ptr** — The pointer.

## 반환 값

The new reference count.

## 예제

```ahk
ObjRelease p
```

## 관련

[`ObjPtrAddRef`](ObjPtrAddRef.md), [`ObjFromPtr`](ObjFromPtr.md)
