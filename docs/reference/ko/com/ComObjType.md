# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · COM 자동화 (ComObject)

## 설명

Reads the type/name of a COM object.

## 구문

```ahk
S := ComObjType(Obj, What)
```

## 매개변수

- **Obj** — The object.
- **What** — Name/Class/IID/...

## 반환 값

The requested string.

## 예제

```ahk
n := ComObjType(o, "Name")
```

## 관련

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
