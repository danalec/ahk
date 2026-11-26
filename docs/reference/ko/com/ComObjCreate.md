# `ComObjCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · COM 자동화 (ComObject)

## 설명

Creates a COM object (v1).

## 구문

```ahk
Obj := ComObjCreate(CLSID)
```

## 매개변수

1..2.

## 반환 값

The object.

## 예제

```ahk
o := ComObjCreate("Shell.Application")
```

## 관련

[`ComObject`](ComObject.md)
