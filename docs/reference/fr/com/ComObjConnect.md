# `ComObjConnect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Connects a COM object's events to script functions.

## Syntaxe

```ahk
ComObjConnect Obj, Prefix
```

## Paramètres

- **Obj** — The object.
- **Prefix** — The function prefix.

## Valeur de retour

Nothing; the event sink refuses on Linux.

## Exemple

```ahk
ComObjConnect ie, "IE_"
```

## Voir aussi

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
