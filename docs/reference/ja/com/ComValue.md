# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · class · COM 自動化 (ComObject)

## 説明

Wraps a raw value as a VARIANT of a given type.

## 構文

```ahk
V := ComValue(Type, Value)
```

## パラメータ

- **Type** — The VARTYPE.
- **Value** — The value.

## 戻り値

A COM value.

## 例

```ahk
v := ComValue(0x4000, 1)
```

## 関連

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
