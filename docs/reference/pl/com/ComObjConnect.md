# `ComObjConnect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Automatyzacja COM (ComObject)

## Opis

Connects a COM object's events to script functions.

## Składnia

```ahk
ComObjConnect Obj, Prefix
```

## Parametry

- **Obj** — The object.
- **Prefix** — The function prefix.

## Wartość zwracana

Nothing; the event sink refuses on Linux.

## Przykład

```ahk
ComObjConnect ie, "IE_"
```

## Powiązane

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
