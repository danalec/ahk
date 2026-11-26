# `ComObjGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · COM 자동화 (ComObject)

## 설명

Gets a COM object from a moniker (a file/URL/ProgID).

## 구문

```ahk
Obj := ComObjGet(Name)
```

## 매개변수

- **Name** — The moniker.

## 반환 값

The object.

## 예제

```ahk
o := ComObjGet("winmgmts:")
```

## 관련

[`ComObject`](ComObject.md), [`ComObjActive`](ComObjActive.md)
