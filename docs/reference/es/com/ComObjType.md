# `ComObjType`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Reads the type/name of a COM object.

## Sintaxis

```ahk
S := ComObjType(Obj, What)
```

## Parámetros

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Valor de retorno

The requested string.

## Ejemplo

```ahk
n := ComObjType(o, "Name")
```

## Relacionados

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
