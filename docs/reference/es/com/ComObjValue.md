# `ComObjValue`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Reads the underlying pointer/value of a COM wrapper.

## Sintaxis

```ahk
P := ComObjValue(Obj)
```

## Parámetros

- **Obj** — The wrapper.

## Valor de retorno

The pointer or value.

## Ejemplo

```ahk
p := ComObjValue(o)
```

## Relacionados

[`ComObjFlags`](ComObjFlags.md), [`ObjPtr`](ObjPtr.md)
