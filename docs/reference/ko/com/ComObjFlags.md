# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · COM 자동화 (ComObject)

## 설명

Gets/sets the flags of a COM wrapper.

## 구문

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## 매개변수

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## 반환 값

The flags.

## 예제

```ahk
f := ComObjFlags(o)
```

## 관련

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
