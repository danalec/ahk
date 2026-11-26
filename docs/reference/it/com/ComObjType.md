# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Automazione COM (ComObject)

## Descrizione

Reads the type/name of a COM object.

## Sintassi

```ahk
S := ComObjType(Obj, What)
```

## Parametri

- **Obj** — The object.
- **What** — Name/Class/IID/...

## Valore di ritorno

The requested string.

## Esempio

```ahk
n := ComObjType(o, "Name")
```

## Correlati

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
