# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · class · Automatyzacja COM (ComObject)

## Opis

Wraps a raw value as a VARIANT of a given type.

## Składnia

```ahk
V := ComValue(Type, Value)
```

## Parametry

- **Type** — The VARTYPE.
- **Value** — The value.

## Wartość zwracana

A COM value.

## Przykład

```ahk
v := ComValue(0x4000, 1)
```

## Powiązane

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
