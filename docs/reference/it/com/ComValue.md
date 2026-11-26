# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · class · Automazione COM (ComObject)

## Descrizione

Wraps a raw value as a VARIANT of a given type.

## Sintassi

```ahk
V := ComValue(Type, Value)
```

## Parametri

- **Type** — The VARTYPE.
- **Value** — The value.

## Valore di ritorno

A COM value.

## Esempio

```ahk
v := ComValue(0x4000, 1)
```

## Correlati

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
