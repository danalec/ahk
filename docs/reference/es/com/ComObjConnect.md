# `ComObjConnect`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Connects a COM object's events to script functions.

## Sintaxis

```ahk
ComObjConnect Obj, Prefix
```

## Parámetros

- **Obj** — The object.
- **Prefix** — The function prefix.

## Valor de retorno

Nothing; the event sink refuses on Linux.

## Ejemplo

```ahk
ComObjConnect ie, "IE_"
```

## Relacionados

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
