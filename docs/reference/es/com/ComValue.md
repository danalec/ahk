# `ComValue`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · class · Automatización COM (ComObject)

## Descripción

Wraps a raw value as a VARIANT of a given type.

## Sintaxis

```ahk
V := ComValue(Type, Value)
```

## Parámetros

- **Type** — The VARTYPE.
- **Value** — The value.

## Valor de retorno

A COM value.

## Ejemplo

```ahk
v := ComValue(0x4000, 1)
```

## Relacionados

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
