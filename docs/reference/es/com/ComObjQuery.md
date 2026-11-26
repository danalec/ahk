# `ComObjQuery`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Queries another interface of a COM object.

## Sintaxis

```ahk
Obj := ComObjQuery(Obj, IID)
```

## Parámetros

- **Obj** — The object.
- **IID** — The interface.

## Valor de retorno

The interface object.

## Ejemplo

```ahk
q := ComObjQuery(o, "{...}")
```

## Relacionados

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
