# `ComObject`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · class · Automazione COM (ComObject)

## Descrizione

Creates or gets a COM object (Windows).

## Sintassi

```ahk
Obj := ComObject(CLSID, IID)
```

## Parametri

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## Valore di ritorno

A COM object.

## Osservazioni

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## Esempio

```ahk
shell := ComObject("Shell.Application")
```

## Correlati

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
