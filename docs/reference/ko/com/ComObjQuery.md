# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · COM 자동화 (ComObject)

## 설명

Queries another interface of a COM object.

## 구문

```ahk
Obj := ComObjQuery(Obj, IID)
```

## 매개변수

- **Obj** — The object.
- **IID** — The interface.

## 반환 값

The interface object.

## 예제

```ahk
q := ComObjQuery(o, "{...}")
```

## 관련

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
