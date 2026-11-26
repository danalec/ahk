# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · function · COM 자동화 (ComObject)

## 설명

Calls a COM method by its vtable index.

## 구문

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## 매개변수

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## 반환 값

The return value.

## 설명

On Linux the D-Bus model has no vtable index; refuses with the reason.

## 예제

```ahk
ComCall(0, obj, "Ptr")
```

## 관련

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
