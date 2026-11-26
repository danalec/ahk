# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · class · COM otomasyonu (ComObject)

## Açıklama

Wraps a raw value as a VARIANT of a given type.

## Sözdizimi

```ahk
V := ComValue(Type, Value)
```

## Parametreler

- **Type** — The VARTYPE.
- **Value** — The value.

## Dönüş değeri

A COM value.

## Örnek

```ahk
v := ComValue(0x4000, 1)
```

## İlgili

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
