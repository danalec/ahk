# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · COM 자동화 (ComObject)

## 설명

Wraps a raw value as a VARIANT of a given type.

## 구문

```ahk
V := ComValue(Type, Value)
```

## 매개변수

- **Type** — The VARTYPE.
- **Value** — The value.

## 반환 값

A COM value.

## 예제

```ahk
v := ComValue(0x4000, 1)
```

## 관련

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
