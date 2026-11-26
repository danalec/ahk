# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · class · COM-Automatisierung (ComObject)

## Beschreibung

Wraps a raw value as a VARIANT of a given type.

## Syntax

```ahk
V := ComValue(Type, Value)
```

## Parameter

- **Type** — The VARTYPE.
- **Value** — The value.

## Rückgabewert

A COM value.

## Beispiel

```ahk
v := ComValue(0x4000, 1)
```

## Verwandt

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
