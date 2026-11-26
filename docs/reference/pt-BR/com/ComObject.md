# `ComObject`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · class · Automação COM (ComObject)

## Descrição

Creates or gets a COM object (Windows).

## Sintaxe

```ahk
Obj := ComObject(CLSID, IID)
```

## Parâmetros

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## Valor de retorno

A COM object.

## Observações

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## Exemplo

```ahk
shell := ComObject("Shell.Application")
```

## Relacionados

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
