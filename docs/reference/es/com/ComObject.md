# `ComObject`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · class · Automatización COM (ComObject)

## Descripción

Creates or gets a COM object (Windows).

## Sintaxis

```ahk
Obj := ComObject(CLSID, IID)
```

## Parámetros

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## Valor de retorno

A COM object.

## Observaciones

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## Ejemplo

```ahk
shell := ComObject("Shell.Application")
```

## Relacionados

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
