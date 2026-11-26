# `ComObjGet`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Gets a COM object from a moniker (a file/URL/ProgID).

## Sintaxis

```ahk
Obj := ComObjGet(Name)
```

## Parámetros

- **Name** — The moniker.

## Valor de retorno

The object.

## Ejemplo

```ahk
o := ComObjGet("winmgmts:")
```

## Relacionados

[`ComObject`](ComObject.md), [`ComObjActive`](ComObjActive.md)
