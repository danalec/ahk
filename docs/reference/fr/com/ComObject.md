# `ComObject`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · L'automatisation COM (ComObject)

## Description

Creates or gets a COM object (Windows).

## Syntaxe

```ahk
Obj := ComObject(CLSID, IID)
```

## Paramètres

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## Valeur de retour

A COM object.

## Remarques

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## Exemple

```ahk
shell := ComObject("Shell.Application")
```

## Voir aussi

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
