# `ComValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · class · Automação COM (ComObject)

## Descrição

Wraps a raw value as a VARIANT of a given type.

## Sintaxe

```ahk
V := ComValue(Type, Value)
```

## Parâmetros

- **Type** — The VARTYPE.
- **Value** — The value.

## Valor de retorno

A COM value.

## Exemplo

```ahk
v := ComValue(0x4000, 1)
```

## Relacionados

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
