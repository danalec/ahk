# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · class · COM 自動化 (ComObject)

## 描述

Wraps a raw value as a VARIANT of a given type.

## 語法

```ahk
V := ComValue(Type, Value)
```

## 參數

- **Type** — The VARTYPE.
- **Value** — The value.

## 傳回值

A COM value.

## 範例

```ahk
v := ComValue(0x4000, 1)
```

## 相關

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
